use crate::{edge::Edge, node::Node, shared::*, slot::Slot};
use orbtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use texture_processor::{
    node::{NodeType, Side},
    node_graph::{Edge as CoreEdge, NodeGraph, NodeId, SlotId},
};

#[derive(Default, Serialize, Deserialize)]
struct NodeGraphSpatial {
    locations: Vec<Location>,
    node_graph: NodeGraph,
}

#[derive(Serialize, Deserialize)]
struct Location {
    node_id: NodeId,
    point: (f64, f64),
}

widget!(NodeContainer<NodeContainerState> {
    action: OptionAction,
    action_file: OptionActionFile
});

impl Template for NodeContainer {
    fn template(self, _id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("NodeContainer")
    }
}

#[derive(Default, AsAny)]
struct NodeContainerState {
    node_graph_spatial: NodeGraphSpatial,
    dragged_edges: (Vec<Entity>, WidgetSide),
    mouse_position: Point,
    dragged_entity: OptionDragDropEntity,
    dropped_on_entity: OptionDragDropEntity,
}

impl State for NodeContainerState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.handle_mouse_action(ctx);
        self.handle_dragged_entity(ctx);
        self.handle_dropped_entity(ctx);

        self.reset_mouse_action(ctx);

        self.handle_action_file(ctx);
    }
}

impl NodeContainerState {
    fn reset_mouse_action(&mut self, ctx: &mut Context) {
        if let Some(action) = ctx.widget().get::<OptionAction>("action") {
            if let Action::Release(_) = action {
                self.dragged_entity = None
            }
        }

        ctx.widget().set::<OptionAction>("action", None);
    }

    fn update_dragged_node_to_graph(&mut self, ctx: &mut Context) {
        let dragged_entity = if let Some(drag_drop_entity) = self.dragged_entity {
            if drag_drop_entity.widget_type == WidgetType::Node {
                drag_drop_entity.entity
            } else {
                return;
            }
        } else {
            return;
        };

        self.update_node_to_graph(ctx, dragged_entity);
    }

    fn update_node_to_graph(&mut self, ctx: &mut Context<'_>, entity: Entity) {
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

    fn handle_dropped_entity(&mut self, ctx: &mut Context) {
        if let Some(action) = ctx.widget().get::<OptionAction>("action") {
            match *action {
                Action::Release(_) => {}
                _ => return,
            };
        } else {
            return;
        }

        let dropped_on_entity = match self.dropped_on_entity {
            Some(drag_drop_entity) => drag_drop_entity,
            None => {
                self.remove_dragged_edges(ctx);
                self.update_dragged_node_to_graph(ctx);
                return;
            }
        };

        match dropped_on_entity.widget_type {
            WidgetType::Slot => {
                let dropped_on_widget = ctx.get_widget(dropped_on_entity.entity);

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

                    let (other_node_id, other_slot_id, other_side) = match dropped_on_side {
                        WidgetSide::Input => {
                            edge_widget.set::<u32>("input_node", dropped_on_node_id);
                            edge_widget.set::<u32>("input_slot", dropped_on_slot);
                            edge_widget.set::<Point>("input_point", goal_position);
                            (
                                *edge_widget.get::<u32>("output_node"),
                                *edge_widget.get::<u32>("output_slot"),
                                Side::Output,
                            )
                        }
                        WidgetSide::Output => {
                            edge_widget.set::<u32>("output_node", dropped_on_node_id);
                            edge_widget.set::<u32>("output_slot", dropped_on_slot);
                            edge_widget.set::<Point>("output_point", goal_position);
                            (
                                *edge_widget.get::<u32>("input_node"),
                                *edge_widget.get::<u32>("input_slot"),
                                Side::Input,
                            )
                        }
                    };

                    ctx.push_event(ChangedEvent(edge_entity));
                    let _ = self.node_graph_spatial.node_graph.connect_arbitrary(
                        NodeId(dropped_on_node_id),
                        dropped_on_side.into(),
                        SlotId(dropped_on_slot),
                        NodeId(other_node_id),
                        other_side,
                        SlotId(other_slot_id),
                    );
                }
                self.update_slot_edges_from_graph(ctx, dropped_on_entity.entity);
            }
            WidgetType::Node => {
                panic!("Somehow dropped something on a node, should not be possible")
            }
            WidgetType::Edge => {
                panic!("Somehow dropped something on an edge, should not be possible")
            }
        };

        self.dropped_on_entity = None;
    }

    /// Updates all edges connected to the given `slot_entity` using the data in the graph.
    fn update_slot_edges_from_graph(&mut self, ctx: &mut Context, slot_entity: Entity) {
        self.remove_edges_in_slot(ctx, slot_entity);
        let slot_widget = ctx.get_widget(slot_entity);

        let node_id = *slot_widget.get::<u32>("node_id");
        let slot_id = *slot_widget.get::<u32>("slot_id");
        let side: Side = (*slot_widget.get::<WidgetSide>("side")).into();

        let edges_to_create: Vec<CoreEdge> = self
            .node_graph_spatial
            .node_graph
            .edges_in_slot(NodeId(node_id), side, SlotId(slot_id))
            .iter()
            .map(|(_, edge)| **edge)
            .collect();

        for edge in edges_to_create {
            self.create_edge(ctx, &edge);
        }
    }

    /// Removes all visual edges connected to a slot.
    fn remove_edges_in_slot(&mut self, ctx: &mut Context, slot_entity: Entity) {
        let edge_entities = self.get_edges_in_slot(ctx, slot_entity);

        for edge in edge_entities {
            ctx.remove_child(edge);
        }
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
            ctx.remove_child(dragged_edge_entity);
        }

        self.dragged_edges.0 = Vec::new();
    }

    fn handle_dragged_entity(&mut self, ctx: &mut Context) {
        let dragged_entity = match self.dragged_entity {
            Some(drag_drop_entity) => drag_drop_entity,
            None => return,
        };

        match dragged_entity.widget_type {
            WidgetType::Node => {
                self.refresh_node(ctx, dragged_entity.entity);
            }
            WidgetType::Slot => {
                self.grab_slot_edge(ctx, dragged_entity.entity);
            }
            WidgetType::Edge => {
                self.refresh_dragged_edges(ctx);
            }
        };
    }

    fn entity_type(ctx: &mut Context, entity: Entity, widget_type_input: WidgetType) -> bool {
        if let Some(widget_type) = ctx.get_widget(entity).try_get::<WidgetType>("widget_type") {
            *widget_type == widget_type_input
        } else {
            false
        }
    }

    /// Updates the visual position of the given `Entity` with `WidgetType::Node`.
    fn refresh_node(&mut self, ctx: &mut Context, node_entity: Entity) {
        if !Self::entity_type(ctx, node_entity, WidgetType::Node) {
            return;
        }
        let mut dragged_widget = ctx.get_widget(node_entity);
        let current_margin = *dragged_widget.get::<Thickness>("my_margin");

        dragged_widget.set::<Thickness>(
            "my_margin",
            Thickness {
                left: self.mouse_position.x - NODE_SIZE * 0.5,
                right: current_margin.right,
                top: self.mouse_position.y - NODE_SIZE * 0.5,
                bottom: current_margin.bottom,
            },
        );

        self.refresh_node_edges(ctx, node_entity);
        self.refresh_node_slots(ctx, node_entity);
    }

    fn grab_slot_edge(&mut self, ctx: &mut Context, slot_entity: Entity) {
        let slot_side = *ctx.get_widget(slot_entity).get::<WidgetSide>("side");
        let slot_node_id = *ctx.get_widget(slot_entity).get::<u32>("node_id");
        let slot_id = *ctx.get_widget(slot_entity).get::<u32>("slot_id");

        let mouse_position = Point {
            x: self.mouse_position.x,
            y: self.mouse_position.y,
        };

        let dragged_edges = match slot_side {
            WidgetSide::Input => {
                let dragged_edges = self.get_dragged_edges(ctx);

                if dragged_edges.is_empty() {
                    (
                        vec![self.create_loose_edge(
                            ctx,
                            slot_node_id,
                            slot_side,
                            slot_id,
                            None,
                            None,
                            Some(mouse_position),
                        )],
                        WidgetSide::Output,
                    )
                } else {
                    (dragged_edges, WidgetSide::Input)
                }
            }
            WidgetSide::Output => (
                vec![self.create_loose_edge(
                    ctx,
                    slot_node_id,
                    slot_side,
                    slot_id,
                    None,
                    None,
                    Some(mouse_position),
                )],
                WidgetSide::Input,
            ),
        };

        self.dragged_entity = Some(DragDropEntity::new(WidgetType::Edge, Entity(0)));

        self.dragged_edges = dragged_edges;
        self.refresh_dragged_edges(ctx);
    }

    fn refresh_dragged_edges(&mut self, ctx: &mut Context) {
        let mouse_point = Point {
            x: self.mouse_position.x,
            y: self.mouse_position.y,
        };
        for edge_entity in self.dragged_edges.0.clone() {
            self.move_edge_side(ctx, edge_entity, self.dragged_edges.1, mouse_point);
        }
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

    fn get_dragged_edges(&mut self, ctx: &mut Context) -> Vec<Entity> {
        let dragged_entity = if self.dragged_entity.is_some() {
            self.dragged_entity.unwrap()
        } else {
            return Vec::new();
        };

        match dragged_entity.widget_type {
            WidgetType::Slot => self.get_edges_in_slot(ctx, dragged_entity.entity),
            WidgetType::Edge => self.dragged_edges.0.clone(),
            _ => Vec::new(),
        }
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

        Self::children_type(ctx, WidgetType::Edge)
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

    fn create_loose_edge(
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

        let self_entity = ctx.widget().entity();
        let bc = &mut ctx.build_context();
        let item = match side {
            WidgetSide::Input => Edge::create()
                .id("edge")
                .output_point(other_point.unwrap_or_default())
                .input_point(slot_position)
                .output_node(other_node_id.unwrap_or_default())
                .input_node(node_id)
                .output_slot(other_slot_id.unwrap_or_default())
                .input_slot(slot_id)
                .build(bc),
            WidgetSide::Output => Edge::create()
                .id("edge")
                .output_point(slot_position)
                .input_point(other_point.unwrap_or_default())
                .output_node(node_id)
                .input_node(other_node_id.unwrap_or_default())
                .output_slot(slot_id)
                .input_slot(other_slot_id.unwrap_or_default())
                .build(bc),
        };
        bc.append_child(self_entity, item);

        *Self::children_type(ctx, WidgetType::Edge)
            .iter()
            .rev()
            .next()
            .unwrap()
    }

    /// Visually refreshes all `Edge` widgets connected to the given `Entity` based on what's
    /// seen in the GUI, not from the actual data.
    fn refresh_node_edges(&mut self, ctx: &mut Context, node_entity: Entity) {
        let node_widget = ctx.get_widget(node_entity);
        let node_id = *node_widget.get::<u32>("node_id");
        let edge_entities: Vec<Entity> = self.node_edges(ctx, NodeId(node_id));

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

            let node_point = Self::node_point(ctx, node_entity);

            let mut edge_widget = ctx.get_widget(edge_entity);
            if output_node == node_id {
                edge_widget.set(
                    "output_point",
                    Self::position_edge(WidgetSide::Output, output_slot, node_point),
                );
            } else if input_node == node_id {
                edge_widget.set(
                    "input_point",
                    Self::position_edge(WidgetSide::Input, input_slot, node_point),
                );
            }
        }
    }

    fn node_point(ctx: &mut Context, node_entity: Entity) -> Point {
        let node_widget = ctx.get_widget(node_entity);
        let node_margin = node_widget.get::<Thickness>("my_margin");

        Point {
            x: node_margin.left,
            y: node_margin.top,
        }
    }

    fn refresh_node_slots(&mut self, ctx: &mut Context, node_entity: Entity) {
        if !Self::entity_type(ctx, node_entity, WidgetType::Node) {
            return;
        }
        let node_widget = ctx.get_widget(node_entity);
        let node_margin = *node_widget.get::<Thickness>("margin");

        let node_id = *node_widget.get::<u32>("node_id");
        let slot_entities: Vec<Entity> = self.node_slots(ctx, NodeId(node_id));

        for slot_entity in slot_entities {
            let (slot_id, side) = {
                let slot_widget = ctx.get_widget(slot_entity);
                (
                    *slot_widget.get::<u32>("slot_id"),
                    *slot_widget.get::<WidgetSide>("side"),
                )
            };

            let mut slot_widget = ctx.get_widget(slot_entity);

            slot_widget.set("margin", Self::position_slot(side, slot_id, node_margin));
        }
    }

    fn node_edges(&mut self, ctx: &mut Context, node_id: NodeId) -> Vec<Entity> {
        Self::children_type(ctx, WidgetType::Edge)
            .iter()
            .filter(|entity| {
                let widget = ctx.get_widget(**entity);
                let output_node = *widget.get::<u32>("output_node");
                let input_node = *widget.get::<u32>("input_node");

                output_node == node_id.0 || input_node == node_id.0
            })
            .copied()
            .collect()
    }

    fn node_slots(&mut self, ctx: &mut Context, node_id: NodeId) -> Vec<Entity> {
        Self::children_type(ctx, WidgetType::Slot)
            .iter()
            .filter(|entity| {
                let widget = ctx.get_widget(**entity);
                let slot_node_id = *widget.get::<u32>("node_id");

                slot_node_id == node_id.0
            })
            .copied()
            .collect()
    }

    fn children_type(ctx: &mut Context, widget_type: WidgetType) -> Vec<Entity> {
        let mut output: Vec<Entity> = Vec::new();

        for i in 0.. {
            if let Some(widget) = ctx.try_child_from_index(i) {
                let entity = widget.entity();

                if Self::entity_type(ctx, entity, widget_type) {
                    output.push(entity)
                } else {
                    continue;
                }
            } else {
                break;
            };
        }

        output
    }

    fn populate_slots(&mut self, ctx: &mut Context) {
        for node_entity in Self::child_entities_type(ctx, WidgetType::Node) {
            let self_entity = ctx.widget().entity();
            let node_margin = *ctx.get_widget(node_entity).get::<Thickness>("my_margin");
            let node_id = *ctx.get_widget(node_entity).get::<u32>("node_id");

            for i in 0..*ctx.get_widget(node_entity).get::<usize>("slot_count_input") {
                let build_context = &mut ctx.build_context();

                let slot_margin = Self::position_slot(WidgetSide::Input, i as u32, node_margin);

                let item = Slot::create()
                    .node_id(node_id)
                    .margin(slot_margin)
                    .side(WidgetSide::Input)
                    .slot_id(i as u32)
                    .build(build_context);

                build_context.append_child(self_entity, item);
            }

            for i in 0..*ctx
                .get_widget(node_entity)
                .get::<usize>("slot_count_output")
            {
                let build_context = &mut ctx.build_context();

                let slot_margin = Self::position_slot(WidgetSide::Output, i as u32, node_margin);

                let item = Slot::create()
                    .node_id(node_id)
                    .margin(slot_margin)
                    .side(WidgetSide::Output)
                    .slot_id(i as u32)
                    .build(build_context);

                build_context.append_child(self_entity, item);
            }
        }
    }

    fn position_slot(side: WidgetSide, slot: u32, node_margin: Thickness) -> Thickness {
        let left = node_margin.left - SLOT_SIZE_HALF;
        let top = node_margin.top + ((SLOT_SIZE + SLOT_SPACING) * slot as f64);
        match side {
            WidgetSide::Input => Thickness {
                left,
                top,
                right: 0.,
                bottom: 0.,
            },
            WidgetSide::Output => Thickness {
                left: left + NODE_SIZE,
                top,
                right: 0.,
                bottom: 0.,
            },
        }
    }

    fn child_entities_type(ctx: &mut Context, widget_type: WidgetType) -> Vec<Entity> {
        child_entities(ctx)
            .iter()
            .filter(|entity| {
                ctx.get_widget(**entity)
                    .try_get::<WidgetType>("widget_type")
                    == Some(&widget_type)
            })
            .cloned()
            .collect()
    }

    fn handle_mouse_action(&mut self, ctx: &mut Context) {
        if let Some(action) = *ctx.widget().get::<OptionAction>("action") {
            match action {
                Action::Press(p) => {
                    for child_entity in child_entities(ctx) {
                        if ctx
                            .get_widget(child_entity)
                            .get::<Rectangle>("bounds")
                            .contains((p.x, p.y))
                            && self.is_clickable(ctx, child_entity)
                        {
                            self.dragged_entity = Some(DragDropEntity {
                                widget_type: *ctx
                                    .get_widget(child_entity)
                                    .get::<WidgetType>("widget_type"),
                                entity: child_entity,
                            });
                        }
                    }

                    self.mouse_position = p;
                }
                Action::Release(p) => {
                    let widget_type = WidgetType::Slot;

                    for slot_entity in Self::children_type(ctx, widget_type) {
                        if ctx
                            .get_widget(slot_entity)
                            .get::<Rectangle>("bounds")
                            .contains((p.x, p.y))
                        {
                            self.dropped_on_entity = Some(DragDropEntity {
                                widget_type,
                                entity: slot_entity,
                            });
                        }
                    }

                    self.mouse_position = p;
                }
                Action::Move(p) => self.mouse_position = p,
                Action::Scroll(p) => self.mouse_position = p,
            }
        }
    }

    fn is_clickable(&mut self, ctx: &mut Context, entity: Entity) -> bool {
        if let Some(widget_type) = ctx.get_widget(entity).try_get("widget_type") {
            match widget_type {
                WidgetType::Node => true,
                WidgetType::Slot => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn handle_action_file(&mut self, ctx: &mut Context<'_>) {
        if let Some(action_file) = ctx.widget().get::<OptionActionFile>("action_file").clone() {
            match action_file {
                ActionFile::LoadGraph(path) => {
                    self.load_graph(ctx, path);
                }
                ActionFile::SaveGraph(path) => {
                    self.save_graph(path);
                }
            };

            ctx.widget().set::<OptionActionFile>("action_file", None);
        }
    }

    fn populate_workspace(&mut self, ctx: &mut Context<'_>) {
        ctx.clear_children();

        self.populate_nodes(ctx);
        self.populate_slots(ctx);
        self.populate_edges(ctx);
    }

    fn populate_nodes(&mut self, ctx: &mut Context) {
        let self_entity = ctx.widget().entity();
        let bc = &mut ctx.build_context();

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

            let item = Node::create()
                .id(node.node_id.0.to_string())
                .title(node_title)
                .node_id(node.node_id.0)
                .my_margin(margin)
                .slot_count_input(slot_count_input)
                .slot_count_output(slot_count_output)
                .build(bc);

            bc.append_child(self_entity, item);
        }
    }

    fn populate_edges(&mut self, ctx: &mut Context) {
        for edge in self.node_graph_spatial.node_graph.edges.clone() {
            self.create_edge(ctx, &edge);
        }
    }

    fn create_edge(&mut self, ctx: &mut Context, edge: &CoreEdge) {
        let self_entity = ctx.widget().entity();
        let bc = &mut ctx.build_context();

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

        let output_point = Self::position_edge(WidgetSide::Output, output_slot, output_node_pos);
        let input_point = Self::position_edge(WidgetSide::Input, input_slot, input_node_pos);

        let item = Edge::create()
            .id("edge")
            .output_point(output_point)
            .input_point(input_point)
            .output_node(edge.output_id.0)
            .input_node(edge.input_id.0)
            .output_slot(output_slot)
            .input_slot(input_slot)
            .build(bc);

        bc.append_child(self_entity, item);
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

    fn load_graph(&mut self, ctx: &mut Context<'_>, path: String) {
        let file = File::open(path).unwrap();
        self.node_graph_spatial = serde_json::from_reader(file).unwrap();

        self.dragged_edges.0 = Vec::new();
        self.populate_workspace(ctx);
    }

    fn save_graph(&mut self, path: String) {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(&file, &self.node_graph_spatial).unwrap();
    }
}
