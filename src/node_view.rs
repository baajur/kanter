use orbtk::prelude::*;

use crate::node_state::NodeState;

widget!(
    NodeView<NodeState>: MouseHandler {
        title: String16,
        my_margin: Thickness
    }
);

impl Template for NodeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NodeView")
            .width(100.)
            .height(100.)
            .margin(("my_margin", id))
            .child(
                Container::create()
                    .background(Color::rgb(0, 255, 0))
                    .border_width(1.)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
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
    }
}
