use orbtk::prelude::*;
// use orbtk::behaviors::MouseBehavior;

use crate::node_container_state::NodeContainerState;
// use crate::node_view::NodeView;

widget!(
    NodeContainerView<NodeContainerState> {
        count: usize
    }
);

impl Template for NodeContainerView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // let my_margin_a = Thickness{
        //     left: 10.,
        //     top: 0.,
        //     right: 0.,
        //     bottom: 0.,
        // };
        self.name("NodeContainerView")
            .id("node_container_view")
            .child(Container::create().id("node_container").build(ctx))
    }
}
