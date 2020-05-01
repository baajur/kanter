use crate::shared::*;
use orbtk::{behaviors::MouseBehavior, prelude::*};

widget!(
    Node<NodeState>: MouseHandler {
        widget_type: WidgetType,
        title: String16,
        my_margin: Thickness,
        node_id: u32,
        slot_count_input: usize,
        slot_count_output: usize
    }
);

impl Template for Node {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Node")
            .widget_type(WidgetType::Node)
            .width(NODE_SIZE)
            .height(NODE_SIZE)
            .margin(("my_margin", id))
            .child(MouseBehavior::create().enabled(id).target(id.0).build(ctx))
            .child(
                Container::create()
                    .background(Color::rgb(0, 255, 0))
                    .border_width(1.)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
                    .child(
                        TextBlock::create()
                            .id("title")
                            .text(("title", id))
                            .element("text-block")
                            .horizontal_alignment("center")
                            .foreground(Color::rgb(255, 0, 0))
                            .width(0.)
                            .height(14.)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pub mouse_action: Option<MouseAction>,
    pub builder: WidgetBuildContext,
}

impl State for NodeState {}
