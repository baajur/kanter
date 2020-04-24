use crate::node_view::NodeView;
use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeWorkspaceState {
    pub builder: WidgetBuildContext,
    count: usize,
    node_workspace: Entity,
    mouse_position: (f64, f64),
    mouse_down: bool,
}

impl State for NodeWorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.node_workspace = ctx
            .entity_of_child("node_workspace")
            .expect("`node_workspace` child could not be found.");
        self.mouse_down = false;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let count = ctx.widget().clone_or_default::<usize>("count");

        if count != self.count {
            let build_context = &mut ctx.build_context();
            let item = NodeView::create().title("My node").build(build_context);
            build_context.append_child(self.node_workspace, item);

            self.count = count;
        }

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
}
