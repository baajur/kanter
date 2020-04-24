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
    pub title: String16,
    pub action: Option<Action>,
    mouse_state: MouseState,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            title: String16::default(),
            action: None,
            mouse_state: MouseState::MouseUp,
        }
    }
}

impl State for NodeState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::MousePressed => {
                    if self.mouse_state == MouseState::MouseUp {
                        let entity = ctx.widget().entity();
                        ctx.parent_from_id("node_workspace")
                            .set("dragged_node", Some(entity));
                        self.mouse_state = MouseState::MouseDown;
                    }
                }
                Action::MouseReleased => {
                    if self.mouse_state == MouseState::MouseDown {
                        self.mouse_state = MouseState::MouseUp;
                    }
                }
            }
        }
    }
}

impl NodeState {
    pub fn action(&mut self, action: Action) {
        self.action = Some(action);
    }
}
