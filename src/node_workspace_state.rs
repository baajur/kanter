use crate::node_view::NodeView;
use orbtk::prelude::*;
use texture_processor::node_graph::NodeGraph;

#[derive(Default, AsAny)]
pub struct NodeWorkspaceState {
    pub builder: WidgetBuildContext,
    node_workspace: Entity,
    mouse_position: (f64, f64),
    mouse_down: bool,
}

// struct Location {
//     node_id: NodeId,
//     point: Point,
// }

// struct NodeGraphSpatial {
//     locations: Vec<Location>,
//     node_graph: NodeGraph,
// }

impl State for NodeWorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.node_workspace = ctx
            .entity_of_child("node_workspace")
            .expect("`node_workspace` child could not be found.");
        self.mouse_down = false;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !self.mouse_down && ctx.widget().get::<Option<Entity>>("dragged_node").is_some() {
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

        if !ctx.widget().get::<String16>("load_graph").is_empty() {
            self.load_graph(ctx);
            ctx.widget().set::<String16>("load_graph", String16::new());
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

    fn load_graph(&mut self, ctx: &mut Context) {
        let path = ctx.widget().get::<String16>("load_graph").to_string();
        let node_graph = NodeGraph::from_path(path).unwrap();

        ctx.clear_children();

        for node in node_graph.nodes() {
            let build_context = &mut ctx.build_context();
            let node_title = format!("{:?}", node.node_type);
            let item = NodeView::create().title(node_title).build(build_context);
            build_context.append_child(self.node_workspace, item);
        }
    }

    fn save_graph(&mut self, ctx: &mut Context) {
        let path = ctx.widget().get::<String16>("load_graph").to_string();
        let node_graph = NodeGraph::from_path(path).unwrap();

        ctx.clear_children();

        for node in node_graph.nodes() {
            let build_context = &mut ctx.build_context();
            let node_title = format!("{:?}", node.node_type);
            let item = NodeView::create().title(node_title).build(build_context);
            build_context.append_child(self.node_workspace, item);
        }
    }
}
