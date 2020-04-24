use crate::node_view::NodeView;
use orbtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use texture_processor::node_graph::{NodeGraph, NodeId};

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

        let node_id = widget.get::<String16>("node_id").to_string();
        let node_id: NodeId = NodeId(node_id.parse().unwrap());

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

            let item = NodeView::create()
                .title(node_title)
                .node_id(node.node_id.0.to_string())
                .my_margin(margin)
                .build(build_context);

            build_context.append_child(self.node_workspace, item);
        }
    }
}
