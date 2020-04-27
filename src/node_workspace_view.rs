use crate::{node_workspace_state::NodeWorkspaceState, shared::*};
use orbtk::prelude::*;

pub type DragDropEntity = Option<WidgetType>;

widget!(
    NodeWorkspaceView<NodeWorkspaceState>: MouseHandler {
        count: usize,
        dragged_entity: DragDropEntity,
        dropped_on_entity: DragDropEntity,
        path_load: String16,
        path_save: String16
    }
);

impl Template for NodeWorkspaceView {
    fn template(self, id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("NodeWorkspaceView")
            .id("node_workspace")
            .on_mouse_move(move |states, p| {
                states
                    .get_mut::<NodeWorkspaceState>(id)
                    .mouse_moved(p.x, p.y);
                false
            })
            .on_mouse_down(move |states, _| {
                states
                    .get_mut::<NodeWorkspaceState>(id)
                    .mouse_action(MouseAction::MousePressed);
                false
            })
            .on_mouse_up(move |states, _| {
                states
                    .get_mut::<NodeWorkspaceState>(id)
                    .mouse_action(MouseAction::MouseReleased);
                false
            })
    }
}
