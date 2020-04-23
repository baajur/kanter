use crate::node_view::NodeView;
use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeContainerState {
    pub builder: WidgetBuildContext,
    count: usize,
    node_container: Entity,
}

impl State for NodeContainerState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.node_container = ctx
            .entity_of_child("node_container")
            .expect("ListViewState.init: ItemsPanel child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        // let count = 5;
        let count = ctx.widget().clone_or_default::<usize>("count");

        if count != self.count {
            println!("ja 1");
            ctx.clear_children_of(self.node_container);

            for i in 0..count {
                let my_margin_a = Thickness {
                    left: 50. + 110. * i as f64,
                    top: 30.,
                    right: 0.,
                    bottom: 0.,
                };

                let build_context = &mut ctx.build_context();
                let item = NodeView::create()
                    .my_margin(my_margin_a)
                    .title("My node")
                    .build(build_context);
                build_context.append_child(self.node_container, item);
            }

            self.count = count;
        }
    }
}
