use crate::{line_render_object::LineRenderObject, line_state::LineState};
use orbtk::prelude::*;

widget!(
    LineView<LineState> {
        start_point: Point,
        end_point: Point
    }
);

impl Template for LineView {
    fn template(self, _id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("LineView")
            .start_point(Point { x: 0., y: 0. })
            .end_point(Point { x: 0., y: 0. })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(LineRenderObject)
    }
}
