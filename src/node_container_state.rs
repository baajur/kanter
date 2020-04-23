use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeContainerState {
    builder: WidgetBuildContext,
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
        let count = ctx.widget().clone_or_default::<usize>("count");

        if count != self.count {
            if let Some(builder) = &self.builder {
                ctx.clear_children_of(self.node_container);

                for i in 0..count {
                    let build_context = &mut ctx.build_context();
                    let item = ListViewItem::create().build(build_context);
                    build_context.append_child(self.node_container, item);
                }
            }

            self.count = count;
        }
    }
}