use crate::{
    line_view::LineView,
    edge_state::EdgeState,
};
use orbtk::prelude::*;

widget!(
    EdgeView<EdgeState> {
        start_point: Point,
        end_point: Point,
        start_node: u32,
        end_node: u32
    }
);

impl Template for EdgeView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("EdgeView")
            .id("edge")
            .child(
                LineView::create()
                    .start_point(("start_point", id))
                    .end_point(("end_point", id))
                    .build(ctx)
            )
    }
}
