use crate::shared::*;
use orbtk::prelude::*;

#[derive(AsAny)]
pub struct SlotState {
    pub mouse_action: Option<MouseAction>,
    mouse_state: MouseState,
}

impl Default for SlotState {
    fn default() -> Self {
        Self {
            mouse_action: None,
            mouse_state: MouseState::MouseUp,
        }
    }
}

impl State for SlotState {}

impl SlotState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction) {
        self.mouse_action = Some(mouse_action);
    }
}
