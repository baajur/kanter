use orbtk::prelude::*;
use std::cell::Cell;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    mouse_down: Cell<bool>,
    my_margin: Thickness,
    // pub pos_x: f64,
    // pub pos_y: f64,
}

impl State for NodeState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if self.mouse_down.get() {
            println!("mouse_down");
            let current_margin = *ctx.widget().get::<Thickness>("my_margin");

            ctx.widget().set::<Thickness>(
                "my_margin",
                Thickness {
                    left: current_margin.left,
                    right: current_margin.right,
                    top: current_margin.top + 10.,
                    bottom: current_margin.bottom,
                },
            );
        }
    }
}

impl NodeState {
    pub fn set_mouse_down(&self, input: bool) {
        self.mouse_down.set(input);
    }
}
