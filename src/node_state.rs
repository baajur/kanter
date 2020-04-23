use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pub pos_x: f64,
    pub pos_y: f64,
}

impl State for NodeState {
    fn update(&mut self, _: &mut Registry, _ctx: &mut Context<'_>) {
        // let margin = (*ctx.widget().get::<f64>("pos_x"), 0., 0., 0.);
        // ctx.child("container").set("margin", margin);
    }
}