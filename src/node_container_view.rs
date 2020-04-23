use orbtk::prelude::*;
// use orbtk::behaviors::MouseBehavior;

use crate::node_container_state::NodeContainerState;
use crate::node_view::NodeView;


widget!(
    NodeContainerView<NodeContainerState> {
    }
);

impl Template for NodeContainerView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let my_margin_a = Thickness{
            left: 10.,
            top: 0.,
            right: 0.,
            bottom: 0.,
        };
        self.name("NodeContainerView")
            .child(
                Container::create()
                    .id("node_container")
                    .child(
                        NodeView::create()
                        .title("My node")
                        .my_margin(my_margin_a)
                        .build(ctx),
                    )
                .build(ctx)
            )
        // let my_margin_a = Thickness{
        //     left: 10.,
        //     top: 0.,
        //     right: 0.,
        //     bottom: 0.,
        // };
        // let my_margin_b = Thickness{
        //     left: 130.,
        //     top: 30.,
        //     right: 0.,
        //     bottom: 0.,
        // };
        // self.name("NodeContainerView")
        //     .child(
        //         Container::create()
        //             .child(
        //                 NodeView::create()
        //                 .title("My node")
        //                 .my_margin(my_margin_a)
        //                 .build(ctx),
        //             )
        //         .build(ctx)
        //     )
        //     .child(
        //         Container::create()
        //             .child(
        //                 NodeView::create()
        //                 .title("My node 2")
        //                 .my_margin(my_margin_b)
        //                 .build(ctx),
        //             )
        //         .build(ctx)
        //     )
    }
}
