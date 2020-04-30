use orbtk::prelude::*;
use crate::{
    shared::*,
    node::Node,
    edge::Edge,
};
use texture_processor::{
    node::{NodeType, Side},
    node_graph::{Edge as CoreEdge, NodeGraph, NodeId},
};
use std::fs::File;
use serde::{Deserialize, Serialize};

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
    clicked_entity: Entity,
    node_graph_spatial: NodeGraphSpatial,
    dragged_edges: (Vec<CoreEdge>, WidgetSide),
}

impl State for NodeContainerState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        // self.handle_dragged_entity(ctx);
        // self.handle_dropped_entity(ctx);

        self.get_clicked_child_entity(ctx);
        ctx.widget().set::<OptionAction>("action", None);

        self.handle_action_file(ctx);
    }
}

impl NodeContainerState {
    fn get_clicked_child_entity(&mut self, ctx: &mut Context) {
        if let Some(action) = *ctx.widget().get::<OptionAction>("action") {
            match action {
                Action::Press(p) => {
                    for child_entity in get_child_entities(ctx) {
                        // dbg!(ctx.get_widget(child_entity).get::<Rectangle>("bounds"));
                        // dbg!(p);
                        if ctx.get_widget(child_entity).get::<Rectangle>("bounds").contains((p.x, p.y))
                            && self.is_clickable(ctx, child_entity) {
                            self.clicked_entity = child_entity;
                            println!("clicked entity: {:?}", self.clicked_entity);
                        }
                    }
                }
                _ => {}
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
                    self.load_graph(ctx, path.to_string());
                }
                ActionFile::SaveGraph(path) => {
                    self.save_graph(path.to_string());
                }
            };

            ctx.widget().set::<OptionActionFile>("action_file", None);
        }
    }

    fn populate_workspace(&mut self, ctx: &mut Context<'_>) {
        ctx.clear_children();

        self.populate_nodes(ctx);
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