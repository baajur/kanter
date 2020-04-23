use orbtk::prelude::*;
// use orbtk::behaviors::MouseBehavior;

use crate::{node_container_state::NodeContainerState, shared::*};
// use crate::node_view::NodeView;

widget!(
    NodeContainerView<NodeContainerState> {
    }
);

impl Template for NodeContainerView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        // let my_margin_a = Thickness{
        //     left: 10.,
        //     top: 0.,
        //     right: 0.,
        //     bottom: 0.,
        // };
        self.name("NodeContainerView")
            .child(Container::create().id("node_container").build(ctx))
    }
}
