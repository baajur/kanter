use orbtk::prelude::*;

pub struct LineRenderObject;

impl RenderObject for LineRenderObject {
    fn render_self(&self, ctx: &mut Context<'_>, global_position: &Point) {
        let (start_point, end_point) = {
            let widget = ctx.widget();
            (
                *widget.get::<Point>("start_point"),
                *widget.get::<Point>("end_point"),
            )
        };

        let rc2d = ctx.render_context_2_d();
        rc2d.begin_path();
        rc2d.set_line_width(3.);
        rc2d.set_stroke_style(Brush::SolidColor(Color::rgb(0, 0, 0)));
        rc2d.move_to(
            global_position.x + start_point.x,
            global_position.y + start_point.y,
        );
        rc2d.line_to(
            global_position.x + end_point.x,
            global_position.y + end_point.y,
        );
        rc2d.stroke();
    }
}
