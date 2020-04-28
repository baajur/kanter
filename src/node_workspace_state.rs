use crate::{
    edge_view::EdgeView,
    node_view::{NodeView, NODE_SIZE},
    node_workspace_view::DragDropEntity,
    shared::*,
    slot_view::{SLOT_SIZE, SLOT_SIZE_HALF, SLOT_SPACING},
};
use orbtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use texture_processor::{
    node::{NodeType, Side},
    node_graph::{NodeGraph, NodeId, SlotId},
};

#[derive(Default, AsAny)]
pub struct NodeWorkspaceState {
    pub builder: WidgetBuildContext,
    node_workspace: Entity,
    mouse_position: (f64, f64),
    mouse_action: Option<MouseAction>,
    mouse_action_previous: Option<MouseAction>,
    node_graph_spatial: NodeGraphSpatial,
    most_recently_dragged: Option<WidgetType>,
}

#[derive(Serialize, Deserialize)]
struct Location {
    node_id: NodeId,
    point: (f64, f64),
}

#[derive(Default, Serialize, Deserialize)]
struct NodeGraphSpatial {
    locations: Vec<Location>,
    node_graph: NodeGraph,
}

impl State for NodeWorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.node_workspace = ctx
            .entity_of_child("node_workspace")
            .expect("`node_workspace` child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.handle_dragged_entity(ctx);
        self.handle_dropped_entity(ctx);

        self.mouse_action_previous = self.mouse_action;
        self.mouse_action = None;

        if !ctx.widget().get::<String16>("path_load").is_empty() {
            self.load_graph(ctx);
            ctx.widget().set::<String16>("path_load", String16::new());
        }

        if !ctx.widget().get::<String16>("path_save").is_empty() {
            self.save_graph(ctx);
            ctx.widget().set::<String16>("path_save", String16::new());
        }
    }

    // fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {

    // }
}

impl NodeWorkspaceState {
    pub fn mouse_moved(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }

    pub fn mouse_action(&mut self, mouse_action: MouseAction) {
        self.mouse_action = Some(mouse_action);
    }

    fn handle_dragged_entity(&mut self, ctx: &mut Context) {
        let dragged_entity = *ctx.widget().get::<DragDropEntity>("dragged_entity");

        if dragged_entity.is_some() {
            self.most_recently_dragged = dragged_entity;
        }

        if self.mouse_action == Some(MouseAction::MouseReleased) {
            ctx.widget().set::<DragDropEntity>("dragged_entity", None);
        }

        match dragged_entity {
            Some(WidgetType::Node(held_entity)) => {
                // Update the visual position of the node.
                let mut held_widget = ctx.get_widget(held_entity);
                let current_margin = *held_widget.get::<Thickness>("my_margin");

                held_widget.set::<Thickness>(
                    "my_margin",
                    Thickness {
                        left: self.mouse_position.0 - NODE_SIZE * 0.5,
                        right: current_margin.right,
                        top: self.mouse_position.1 - NODE_SIZE * 0.5,
                        bottom: current_margin.bottom,
                    },
                );

                self.refresh_edges_of_node(ctx, held_entity);
            }
            Some(WidgetType::Slot(held_entity)) => {
                self.grab_slot_edge(ctx, held_entity);
                // TODO: Update slot in node_graph
            }
            _ => {}
        };
    }

    fn create_new_edge(
        &mut self,
        ctx: &mut Context,
        node_id: u32,
        side: WidgetSide,
        slot_id: u32,
        other_node_id: Option<u32>,
        other_slot_id: Option<u32>,
        other_point: Option<Point>,
    ) -> Entity {
        let node_margin = *ctx
            .child(&*node_id.to_string())
            .get::<Thickness>("my_margin");
        let node_pos = Point {
            x: node_margin.left,
            y: node_margin.top,
        };
        let slot_position = Self::position_edge(side, slot_id, node_pos);

        let bc = &mut ctx.build_context();
        let item = match side {
            WidgetSide::Input => EdgeView::create()
                .id("edge")
                .output_point(other_point.unwrap_or_default())
                .input_point(slot_position)
                .output_node(other_node_id.unwrap_or_default())
                .input_node(node_id)
                .output_slot(other_slot_id.unwrap_or_default())
                .input_slot(slot_id)
                .build(bc),
            WidgetSide::Output => EdgeView::create()
                .id("edge")
                .output_point(slot_position)
                .input_point(other_point.unwrap_or_default())
                .output_node(node_id)
                .input_node(other_node_id.unwrap_or_default())
                .output_slot(slot_id)
                .input_slot(other_slot_id.unwrap_or_default())
                .build(bc),
        };
        bc.append_child(self.node_workspace, item);

        *self.get_child_edges(ctx).iter().rev().next().unwrap()
    }

    fn move_edge_side(
        &mut self,
        ctx: &mut Context,
        edge_entity: Entity,
        side: WidgetSide,
        position: Point,
    ) {
        let side_string = match side {
            WidgetSide::Input => "input_point",
            WidgetSide::Output => "output_point",
        };

        ctx.get_widget(edge_entity)
            .set::<Point>(side_string, position);
    }

    fn grab_slot_edge(&mut self, ctx: &mut Context, slot_entity: Entity) {
        let slot_side = *ctx.get_widget(slot_entity).get::<WidgetSide>("side");
        let slot_node_id = *ctx.get_widget(slot_entity).get::<u32>("node_id");
        let slot_id = *ctx.get_widget(slot_entity).get::<u32>("slot_id");

        let mouse_position = Point {
            x: self.mouse_position.0,
            y: self.mouse_position.1,
        };

        // TODO: This currently spams an edge every frame when grabbing an output, going to refactor
        // this so when you grab a slot, it swaps out the slot for the edge your holding, that
        // should solve this issue.
        let edge_entities = match slot_side {
            WidgetSide::Input => {
                let dragged_edges = self.get_dragged_edges(ctx);

                if dragged_edges.is_empty() {
                    vec![self.create_new_edge(
                        ctx,
                        slot_node_id,
                        slot_side,
                        slot_id,
                        None,
                        None,
                        Some(mouse_position),
                    )]
                } else {
                    dragged_edges
                }
            }
            WidgetSide::Output => vec![self.create_new_edge(
                ctx,
                slot_node_id,
                slot_side,
                slot_id,
                None,
                None,
                Some(mouse_position),
            )],
        };

        for edge_entity in edge_entities {
            self.move_edge_side(ctx, edge_entity, slot_side, mouse_position);
        }
    }

    fn handle_dropped_entity(&mut self, ctx: &mut Context) {
        let dragged_entity = *ctx.widget().get::<DragDropEntity>("dragged_entity");

        if self.mouse_action.is_some()
            || self.mouse_action_previous.is_none()
            || dragged_entity.is_some()
        {
            return;
        }

        let dropped_on_entity = *ctx.widget().get::<DragDropEntity>("dropped_on_entity");

        // I'm pretty sure I need to use this dragged entity somewhere to know what slot to connect
        // to what slot.
        let _dragged_entity = self.most_recently_dragged;

        match dropped_on_entity {
            Some(WidgetType::Slot(dropped_on_entity)) => {
                let dropped_on_widget = ctx.get_widget(dropped_on_entity);

                let dropped_on_node_id = *dropped_on_widget.get::<u32>("node_id");
                let dropped_on_side = *dropped_on_widget.get::<WidgetSide>("side");
                let dropped_on_slot = *dropped_on_widget.get::<u32>("slot_id");

                let goal_position = {
                    let node_margin = *ctx
                        .child(&*dropped_on_node_id.to_string())
                        .get::<Thickness>("my_margin");
                    let node_pos = Point {
                        x: node_margin.left,
                        y: node_margin.top,
                    };
                    Self::position_edge(dropped_on_side, dropped_on_slot, node_pos)
                };

                for edge_entity in self.get_dragged_edges(ctx) {
                    let mut edge_widget = ctx.get_widget(edge_entity);

                    match dropped_on_side {
                        WidgetSide::Input => {
                            edge_widget
                                .set::<u32>("input_node", *edge_widget.get::<u32>("input_node"));
                            edge_widget
                                .set::<u32>("input_slot", *edge_widget.get::<u32>("input_slot"));
                            edge_widget.set::<Point>("input_point", goal_position);
                        }
                        WidgetSide::Output => {
                            edge_widget
                                .set::<u32>("output_node", *edge_widget.get::<u32>("output_node"));
                            edge_widget
                                .set::<u32>("output_slot", *edge_widget.get::<u32>("output_slot"));
                            edge_widget.set::<Point>("output_point", goal_position);
                        }
                    };

                    ctx.push_event(ChangedEvent(edge_entity));
                }
            }
            Some(WidgetType::Node(_dropped_on_entity)) => self.update_dragged_node(ctx),
            _ => self.remove_dragged_edges(ctx), // Dragged edges get deleted too early here, how do I know when to actually delete them???
        };

        ctx.widget()
            .set::<DragDropEntity>("dropped_on_entity", None);
    }

    fn get_dragged_edges(&mut self, ctx: &mut Context) -> Vec<Entity> {
        // let slot_entity = match *ctx.widget().get::<DragDropEntity>("dragged_entity") {
        let slot_entity = match self.most_recently_dragged {
            Some(WidgetType::Slot(entity)) => entity,
            _ => return Vec::new(),
        };

        self.get_edges_in_slot(ctx, slot_entity)
    }

    fn get_edges_in_slot(&mut self, ctx: &mut Context, slot_entity: Entity) -> Vec<Entity> {
        let slot_widget = ctx.get_widget(slot_entity);

        let (slot_node_id, slot_id, slot_side) = {
            (
                *slot_widget.get::<u32>("node_id"),
                *slot_widget.get::<u32>("slot_id"),
                *slot_widget.get::<WidgetSide>("side"),
            )
        };

        self.get_child_edges(ctx)
            .iter()
            .filter(|entity| {
                let edge_widget = ctx.get_widget(**entity);

                let (
                    edge_output_node_id,
                    edge_input_node_id,
                    edge_output_slot_id,
                    edge_input_slot_id,
                ) = {
                    (
                        *edge_widget.get::<u32>("output_node"),
                        *edge_widget.get::<u32>("input_node"),
                        *edge_widget.get::<u32>("output_slot"),
                        *edge_widget.get::<u32>("input_slot"),
                    )
                };

                match slot_side {
                    WidgetSide::Input => {
                        slot_node_id == edge_input_node_id && slot_id == edge_input_slot_id
                    }
                    WidgetSide::Output => {
                        slot_node_id == edge_output_node_id && slot_id == edge_output_slot_id
                    }
                }
            })
            .copied()
            .collect()
    }

    fn remove_dragged_edges(&mut self, ctx: &mut Context) {
        let dragged_edge_entities: Vec<Entity> = self.get_dragged_edges(ctx);

        for dragged_edge_entity in dragged_edge_entities {
            let dragged_edge_widget = ctx.get_widget(dragged_edge_entity);

            let (output_node, input_node, output_slot, input_slot): (
                NodeId,
                NodeId,
                SlotId,
                SlotId,
            ) = {
                (
                    NodeId(*dragged_edge_widget.get::<u32>("output_node")),
                    NodeId(*dragged_edge_widget.get::<u32>("input_node")),
                    SlotId(*dragged_edge_widget.get::<u32>("output_slot")),
                    SlotId(*dragged_edge_widget.get::<u32>("input_slot")),
                )
            };

            self.node_graph_spatial.node_graph.remove_edge(
                output_node,
                input_node,
                output_slot,
                input_slot,
            );
            // dbg!(dragged_edge_entity);
            ctx.remove_child(dragged_edge_entity);
            // dbg!(ctx.remove_widget_list());
        }
    }

    fn get_child_edges(&mut self, ctx: &mut Context) -> Vec<Entity> {
        let mut output: Vec<Entity> = Vec::new();

        for i in 0.. {
            let maybe_edge = ctx.try_child_from_index(i);
            if maybe_edge.is_none() {
                break;
            }

            let maybe_edge = maybe_edge.unwrap();

            if let Some(widget_type) = maybe_edge.try_get::<WidgetType>("widget_type") {
                if *widget_type == WidgetType::Edge {
                    let edge_entity = maybe_edge.entity();
                    output.push(edge_entity);
                }
            }
        }

        output
    }

    fn get_edges_of_node(&mut self, ctx: &mut Context, node_id: u32) -> Vec<Entity> {
        self.get_child_edges(ctx)
            .iter()
            .filter(|entity| {
                let widget = ctx.get_widget(**entity);
                let output_node = *widget.get::<u32>("output_node");
                let input_node = *widget.get::<u32>("input_node");

                output_node == node_id || input_node == node_id
            })
            .copied()
            .collect()
    }

    /// Visually refreshes all `EdgeView` widgets connected to the given `Entity` based on what's
    /// seen in the GUI, not from the actual data.
    fn refresh_edges_of_node(&mut self, ctx: &mut Context, node_entity: Entity) {
        let node_widget = ctx.get_widget(node_entity);
        let node_id = *node_widget.get::<u32>("node_id");
        let edge_entities: Vec<Entity> = self.get_edges_of_node(ctx, node_id);

        for edge_entity in edge_entities {
            let (output_node, input_node, output_slot, input_slot) = {
                let edge_widget = ctx.get_widget(edge_entity);
                (
                    *edge_widget.get::<u32>("output_node"),
                    *edge_widget.get::<u32>("input_node"),
                    *edge_widget.get::<u32>("output_slot"),
                    *edge_widget.get::<u32>("input_slot"),
                )
            };

            let node_widget = ctx.child(&*node_id.to_string());
            let node_margin = node_widget.get::<Thickness>("my_margin");
            let node_pos = Point {
                x: node_margin.left,
                y: node_margin.top,
            };

            // let mut child = ctx.child_from_index(i);
            let mut edge_widget = ctx.get_widget(edge_entity);
            if output_node == node_id {
                edge_widget.set(
                    "output_point",
                    Self::position_edge(WidgetSide::Output, output_slot, node_pos),
                );
            } else if input_node == node_id {
                edge_widget.set(
                    "input_point",
                    Self::position_edge(WidgetSide::Input, input_slot, node_pos),
                );
            }
        }
    }

    fn position_edge(side: WidgetSide, slot: u32, node_position: Point) -> Point {
        let x = node_position.x;
        let y = node_position.y + SLOT_SIZE_HALF + ((SLOT_SIZE + SLOT_SPACING) * slot as f64);
        match side {
            WidgetSide::Input => Point { x, y },
            WidgetSide::Output => Point {
                x: x + NODE_SIZE,
                y,
            },
        }
    }

    fn update_dragged_node(&mut self, ctx: &mut Context) {
        let dragged_node_entity = match *ctx.widget().get::<DragDropEntity>("dragged_entity") {
            Some(WidgetType::Node(entity)) => entity,
            _ => return,
        };

        self.update_node(ctx, dragged_node_entity);
    }

    fn update_node(&mut self, ctx: &mut Context<'_>, entity: Entity) {
        let widget = ctx.get_widget(entity);

        let margin = widget.get::<Thickness>("margin");

        let node_id = NodeId(*widget.get::<u32>("node_id"));

        for mut location in &mut self.node_graph_spatial.locations {
            if location.node_id == node_id {
                location.point.0 = margin.left;
                location.point.1 = margin.top;
            }
        }
    }

    // fn load_graph_non_spatial(&mut self, ctx: &mut Context<'_>) {
    //     let path = ctx.widget().get::<String16>("path_load").to_string();
    //     let node_graph = NodeGraph::from_path(path).unwrap();

    //     self.node_graph_spatial = self.node_graph_to_spatial(node_graph);

    //     self.populate_workspace(ctx);
    // }

    fn load_graph(&mut self, ctx: &mut Context<'_>) {
        let path = ctx.widget().get::<String16>("path_load").to_string();
        let file = File::open(path).unwrap();
        self.node_graph_spatial = serde_json::from_reader(file).unwrap();

        self.populate_workspace(ctx);
    }

    fn save_graph(&mut self, ctx: &mut Context) {
        let path = ctx.widget().get::<String16>("path_save").to_string();
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(&file, &self.node_graph_spatial).unwrap();
    }

    // fn node_graph_to_spatial(&mut self, node_graph: NodeGraph) -> NodeGraphSpatial {
    //     let mut locations = Vec::with_capacity(node_graph.nodes().len());

    //     for node in node_graph.nodes() {
    //         locations.push(Location {
    //             node_id: node.node_id,
    //             point: (0., 0.),
    //         });
    //     }

    //     NodeGraphSpatial {
    //         locations,
    //         node_graph,
    //     }
    // }

    fn populate_workspace(&mut self, ctx: &mut Context<'_>) {
        ctx.clear_children();

        let bc = &mut ctx.build_context();
        self.populate_nodes(bc);
        self.populate_edges(bc);
    }

    fn populate_nodes(&mut self, bc: &mut BuildContext) {
        for node in self.node_graph_spatial.node_graph.nodes() {
            let node_title = format!("{:?}", node.node_type);

            let location = self
                .node_graph_spatial
                .locations
                .iter()
                .find(|loc| loc.node_id == node.node_id)
                .unwrap();

            let margin = Thickness {
                left: location.point.0,
                top: location.point.1,
                right: 0.,
                bottom: 0.,
            };

            let slot_count_input = match node.node_type {
                NodeType::InputGray | NodeType::InputRgba => 0,
                _ => node.capacity(Side::Input),
            };
            let slot_count_output = match node.node_type {
                NodeType::OutputGray | NodeType::OutputRgba => 0,
                _ => node.capacity(Side::Output),
            };

            let item = NodeView::create()
                .id(node.node_id.0.to_string())
                .title(node_title)
                .node_id(node.node_id.0)
                .my_margin(margin)
                .slot_count_input(slot_count_input)
                .slot_count_output(slot_count_output)
                .build(bc);

            bc.append_child(self.node_workspace, item);
        }
    }

    fn populate_edges(&mut self, bc: &mut BuildContext) {
        for edge in &self.node_graph_spatial.node_graph.edges {
            let output_node_pos = self
                .node_graph_spatial
                .locations
                .iter()
                .find(|loc| loc.node_id == edge.output_id)
                .expect("Could not find output node location")
                .point;
            let output_node_pos = Point {
                x: output_node_pos.0,
                y: output_node_pos.1,
            };

            let input_node_pos = self
                .node_graph_spatial
                .locations
                .iter()
                .find(|loc| loc.node_id == edge.input_id)
                .expect("Could not find input node location")
                .point;
            let input_node_pos = Point {
                x: input_node_pos.0,
                y: input_node_pos.1,
            };

            let output_slot = edge.output_slot.0;
            let input_slot = edge.input_slot.0;

            let output_point =
                Self::position_edge(WidgetSide::Output, output_slot, output_node_pos);
            let input_point = Self::position_edge(WidgetSide::Input, input_slot, input_node_pos);

            let item = EdgeView::create()
                .id("edge")
                .output_point(output_point)
                .input_point(input_point)
                .output_node(edge.output_id.0)
                .input_node(edge.input_id.0)
                .output_slot(output_slot)
                .input_slot(input_slot)
                .build(bc);

            bc.append_child(self.node_workspace, item);
        }
    }
}
