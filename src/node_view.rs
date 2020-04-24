use crate::node_state::{Action, NodeState};
use orbtk::{behaviors::MouseBehavior, prelude::*};

widget!(
    NodeView<NodeState>: MouseHandler {
        title: String16,
        my_margin: Thickness,
        node_workspace: Entity,
        node_id: u32
    }
);

impl Template for NodeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NodeView")
            .width(100.)
            .height(100.)
            .node_id(("node_id", id))
            .margin(("my_margin", id))
            .on_mouse_down(move |states, _| {
                states.get_mut::<NodeState>(id).action(Action::MousePressed);
                false
            })
            .on_mouse_up(move |states, _| {
                states
                    .get_mut::<NodeState>(id)
                    .action(Action::MouseReleased);
                false
            })
            .child(MouseBehavior::create().enabled(id).target(id.0).build(ctx))
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
