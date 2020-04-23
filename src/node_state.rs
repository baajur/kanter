use orbtk::prelude::*;
use std::cell::Cell;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pressed: Cell<bool>,
    // pub pos_x: f64,
    // pub pos_y: f64,
}

impl State for NodeState {
    fn update(&mut self, _: &mut Registry, _ctx: &mut Context<'_>) {
        if self.pressed.get() {
            println!("Pressed");
            self.pressed.set(false);
        }
    }
}

impl NodeState {
    pub fn pressed(&self) {
        self.pressed.set(true);
    }
}
