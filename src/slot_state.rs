use crate::{node_workspace_view::DragDropEntityType, shared::*};
use orbtk::prelude::*;

#[derive(AsAny)]
pub struct SlotState {
    pub mouse_action: Option<MouseAction>,
}

impl Default for SlotState {
    fn default() -> Self {
        Self { mouse_action: None }
    }
}

impl State for SlotState {
    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(mouse_action) = self.mouse_action {
            match mouse_action {
                MouseAction::MousePressed => {
                    let entity = ctx.widget().entity();

                    ctx.parent_from_id("node_workspace")
                        .set("dragged_entity", Some(DragDropEntityType::Slot(entity)));
                }
                MouseAction::MouseReleased => {}
            }
        }
    }
}

impl SlotState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction) {
        self.mouse_action = Some(mouse_action);
    }
}
