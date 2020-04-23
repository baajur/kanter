use orbtk::prelude::*;

#[derive(Copy, Clone)]
pub enum Action {
    NewNode,
}

#[derive(Default, AsAny)]
pub struct MainState {
    pub action: Option<Action>,
}

impl MainState {
    pub fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

// impl Default for MainState {
//     fn default() -> Self {
//         MainState { Action: None }
//     }
// }

impl State for MainState {
    fn update(&mut self, _: &mut Registry, _ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::NewNode => {
                    println!("New Node");
                }
            }
            self.action = None;
        }
    }
}
