use orbtk::prelude::*;
// use orbtk::behaviors::MouseBehavior;

use crate::node_workspace_state::NodeWorkspaceState;
// use crate::node_view::NodeView;

widget!(
    NodeWorkspaceView<NodeWorkspaceState> {
        count: usize
    }
);

impl Template for NodeWorkspaceView {
    fn template(self, _id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("NodeWorkspaceView").id("node_workspace")
    }
}
