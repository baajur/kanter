use crate::{edge_state::EdgeState, line_view::LineView, shared::*};
use orbtk::prelude::*;

widget!(
    EdgeView<EdgeState> {
        widget_type: WidgetType,
        output_point: Point,
        input_point: Point,
        output_node: u32,
        input_node: u32,
        output_slot: u32,
        input_slot: u32
    }
);

impl Template for EdgeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("EdgeView")
            .id("edge")
            .widget_type(WidgetType::Edge)
            .child(
                LineView::create()
                    .start_point(("output_point", id))
                    .end_point(("input_point", id))
                    .build(ctx),
            )
    }
}
