use orbtk::prelude::*;
use orbtk::behaviors::MouseBehavior;

use crate::node_state::NodeState;


widget!(
    NodeView<NodeState> {
        title: String16
    }
);

impl Template for NodeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NodeView")
            .child(
                Container::create()
                    .background(colors::LINK_WATER_COLOR)
                    .width(100.0)
                    .height(100.0)
                    .child(
                        TextBlock::create()
                            .width(0.0)
                            .height(14.0)
                            .text("Node")
                            .foreground(colors::LYNCH_COLOR)
                            .element("text-block")
                            .vertical_alignment("start")
                            .id("title")
                            .build(ctx),
                        )
                    .build(ctx)
            )
    }
}
