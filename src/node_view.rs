use orbtk::prelude::*;

use crate::{node_state::NodeState, shared::COLOR_TRANSPARENT};

widget!(
    NodeView<NodeState> {
        title: String16,
        my_margin: Thickness
    }
);

impl Template for NodeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NodeView").child(
            Container::create()
                .background(COLOR_TRANSPARENT)
                .child(
                    Container::create()
                        .width(100.)
                        .height(100.)
                        .id("container")
                        .margin(("my_margin", id))
                        .background(COLOR_TRANSPARENT)
                        .child(
                            Container::create()
                                .background(Color::rgb(0, 255, 0))
                                .child(
                                    TextBlock::create()
                                        .width(0.)
                                        .height(14.)
                                        .text(("title", id))
                                        .foreground(Color::rgb(255, 0, 0))
                                        .element("text-block")
                                        .horizontal_alignment("center")
                                        .id("title")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
