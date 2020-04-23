use crate::node_state::NodeState;
use orbtk::{behaviors::MouseBehavior, prelude::*};

widget!(
    NodeView<NodeState>: MouseHandler {
        pressed: bool,
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
            .on_click(move |states, _| {
                states.get::<NodeState>(id).pressed();
                false
            })
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .build(ctx),
            )
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
