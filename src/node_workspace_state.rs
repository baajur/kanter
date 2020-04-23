use crate::node_view::NodeView;
use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeWorkspaceState {
    pub builder: WidgetBuildContext,
    count: usize,
    node_workspace: Entity,
}

impl State for NodeWorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.node_workspace = ctx
            .entity_of_child("node_workspace")
            .expect("`node_workspace` child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let count = ctx.widget().clone_or_default::<usize>("count");

        if count != self.count {
            // ctx.clear_children_of(self.node_workspace);

            // for i in 0..count {
                // let my_margin_a = Thickness {
                //     left: 50. + 110. * i as f64,
                //     top: 50.,
                //     right: 0.,
                //     bottom: 0.,
                // };

                let build_context = &mut ctx.build_context();
                let item = NodeView::create()
                    // .my_margin(my_margin_a)
                    .title("My node")
                    .build(build_context);
                build_context.append_child(self.node_workspace, item);
            // }

            self.count = count;
        }
    }
}
