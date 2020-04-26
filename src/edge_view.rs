use crate::{edge_state::EdgeState, line_view::LineView, shared::*};
use orbtk::prelude::*;

widget!(
    EdgeView<EdgeState> {
        widget_type: WidgetType,
        start_point: Point,
        end_point: Point,
        start_node: u32,
        end_node: u32,
        start_slot: u32,
        end_slot: u32
    }
);

impl Template for EdgeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("EdgeView")
            .id("edge")
            .widget_type(WidgetType::Edge)
            .child(
                LineView::create()
                    .start_point(("start_point", id))
                    .end_point(("end_point", id))
                    .build(ctx),
            )
    }
}
