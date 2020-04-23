use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    // pub pos_x: f64,
    // pub pos_y: f64,
}

impl State for NodeState {
    fn update(&mut self, _: &mut Registry, _ctx: &mut Context<'_>) {}
}
