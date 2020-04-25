use crate::{
    line_view::LineView,
    node_view::{NodeView, NODE_SIZE},
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
        if !self.mouse_down && ctx.widget().get::<Option<Entity>>("dragged_node").is_some() {
            let dragged_entity = ctx.widget().get::<Option<Entity>>("dragged_node").unwrap();
            self.update_node(ctx, dragged_entity);

            ctx.widget().set::<Option<Entity>>("dragged_node", None);
        }

        if let Some(dragged_node_property) = *ctx.widget().get::<Option<Entity>>("dragged_node") {
            let mut dragged_node = ctx.get_widget(dragged_node_property);

            let current_margin = *dragged_node.get::<Thickness>("my_margin");

            dragged_node.set::<Thickness>(
                "my_margin",
                Thickness {
                    left: self.mouse_position.0 - 50.,
                    right: current_margin.right,
                    top: self.mouse_position.1 - 50.,
                    bottom: current_margin.bottom,
                },
            );
        }

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

        for node in self.node_graph_spatial.node_graph.nodes() {
            let build_context = &mut ctx.build_context();
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
                .build(build_context);

            build_context.append_child(self.node_workspace, item);
        }

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

            let output_slot = edge.output_slot.0 as f64;
            let input_slot = edge.input_slot.0 as f64;

            let start_point = Point {
                x: output_node_pos.x + NODE_SIZE,
                y: output_node_pos.y + SLOT_SIZE_HALF + ((SLOT_SIZE + SLOT_SPACING) * output_slot),
            };
            let end_point = Point {
                x: input_node_pos.x,
                y: input_node_pos.y + SLOT_SIZE_HALF + ((SLOT_SIZE + SLOT_SPACING) * input_slot),
            };

            let build_context = &mut ctx.build_context();
            let item = LineView::create()
                .start_point(start_point)
                .end_point(end_point)
                .build(build_context);

            build_context.append_child(self.node_workspace, item);
        }
    }
}
