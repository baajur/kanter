use orbtk::prelude::*;
// use orbtk::behaviors::MouseBehavior;

use crate::node_workspace_state::NodeWorkspaceState;
// use crate::node_view::NodeView;

type DraggedNode = Option<Entity>;

widget!(
    NodeWorkspaceView<NodeWorkspaceState>: MouseHandler {
        count: usize,
        dragged_node: DraggedNode,
        path_load: String16,
        path_save: String16
    }
);

// impl NodeWorkspaceView {
//     pub fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(
//         mut self,
//         builder: F,
//     ) -> Self {
//         self.state_mut().builder = Some(Box::new(builder));
//         self
//     }
// }

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
                states.get_mut::<NodeWorkspaceState>(id).mouse_down();
                false
            })
            .on_mouse_up(move |states, _| {
                states.get_mut::<NodeWorkspaceState>(id).mouse_up();
                false
            })
    }
}
