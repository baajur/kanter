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
    node_graph::{NodeGraph, NodeId},
};

#[derive(Default, AsAny)]
pub struct NodeWorkspaceState {
    pub builder: WidgetBuildContext,
    node_workspace: Entity,
    mouse_position: (f64, f64),
    mouse_down: bool,
    node_graph_spatial: NodeGraphSpatial,
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
        self.mouse_down = false;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        match *ctx.widget().get::<DragDropEntity>("dragged_entity") {
            Some(WidgetType::Node(held_entity)) => {
                // Update the node position in the struct.
                if !self.mouse_down {
                    self.update_node(ctx, held_entity);
                    ctx.widget().set::<DragDropEntity>("dragged_entity", None);
                }

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

                self.refresh_edges(ctx, held_entity);
            }
            Some(WidgetType::Slot(held_entity)) => {
                // Update the edge connection in the struct.
                if !self.mouse_down {}

                // Update the visual location of the edge.
                let held_slot_side = *ctx.get_widget(held_entity).get::<WidgetSide>("side");
                match held_slot_side {
                    WidgetSide::Input => {
                        let mut held_edges: Vec<Entity> = self
                            .get_child_edges(ctx)
                            .iter()
                            .filter(|entity| {
                                let widget = ctx.get_widget(**entity);
                                let edge_input_node = *widget.get::<u32>("input_node");
                                let slot_input_node =
                                    *ctx.get_widget(held_entity).get::<u32>("node_id");

                                edge_input_node == slot_input_node
                            })
                            .copied()
                            .collect();

                        for edge in &mut held_edges {
                            ctx.get_widget(*edge).set::<Point>(
                                "input_point",
                                Point {
                                    x: self.mouse_position.0,
                                    y: self.mouse_position.1,
                                },
                            );
                        }
                    }
                    WidgetSide::Output => todo!(),
                };
            }
            Some(_) => (),
            None => (),
        };

        if !ctx.widget().get::<String16>("path_load").is_empty() {
            self.load_graph(ctx);
            ctx.widget().set::<String16>("path_load", String16::new());
        }

        if !ctx.widget().get::<String16>("path_save").is_empty() {
            self.save_graph(ctx);
            ctx.widget().set::<String16>("path_save", String16::new());
        }
    }
}

impl NodeWorkspaceState {
    pub fn mouse_moved(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }

    pub fn mouse_down(&mut self) {
        self.mouse_down = true;
    }

    pub fn mouse_up(&mut self) {
        self.mouse_down = false;
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

    fn refresh_edges(&mut self, ctx: &mut Context, entity: Entity) {
        let widget = ctx.get_widget(entity);
        let node_id = *widget.get::<u32>("node_id");
        let edges: Vec<Entity> = self.get_edges_of_node(ctx, node_id);

        for edge in edges {
            let (output_node, input_node, output_slot, input_slot) = {
                let edge_widget = ctx.get_widget(edge);
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
            let mut edge_widget = ctx.get_widget(edge);
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
