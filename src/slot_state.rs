use orbtk::prelude::*;

#[derive(Copy, Clone)]
pub enum Action {
    MousePressed,
    MouseReleased,
}

#[derive(PartialEq)]
enum MouseState {
    MouseDown,
    MouseUp,
}

#[derive(AsAny)]
pub struct NodeState {
    pub action: Option<Action>,
    mouse_state: MouseState,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            action: None,
            mouse_state: MouseState::MouseUp,
        }
    }
}

impl State for NodeState {
}

impl NodeState {
    pub fn action(&mut self, action: Action) {
        self.action = Some(action);
    }
}
