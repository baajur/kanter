use orbtk::prelude::*;

use crate::node_state::NodeState;

widget!(
    NodeView<NodeState> {
        title: String16,
        my_margin: Thickness
    }
);

impl Template for NodeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NodeView")
            .child(
                Container::create()
                    .width(100.)
                    .height(100.)
                    .id("container")
                    .margin(("my_margin", id))
                    // .margin((*ctx.get_widget(id).get::<f64>("pos_x"), 10., 0., 0.))
                    .child(
                        Container::create()
                        .background(colors::LINK_WATER_COLOR)
                        .child(
                            TextBlock::create()
                                .width(0.)
                                .height(14.)
                                .text(("title", id))
                                .foreground(colors::LYNCH_COLOR)
                                .element("text-block")
                                .horizontal_alignment("center")
                                .id("title")
                                .build(ctx),
                            )
                        .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
