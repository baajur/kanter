use orbtk::prelude::*;

use crate::{main_state::Action, node_workspace_view::NodeWorkspaceView, MainState};

#[derive(Debug)]
pub struct NodeType {
    pub node_type: String16,
    pub inputs: Vec<u32>,
}

type NodeTypes = Vec<NodeType>;

widget!(
    MainView<MainState> {
        node_types: NodeTypes
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(NodeWorkspaceView::create().build(ctx))
            .child(
                Button::create()
                    .element("button")
                    .on_click(move |states, _| {
                        state(id, states).action(Action::NewNode);
                        true
                    })
                    .build(ctx),
            )
    }
}

fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainState {
    states.get_mut(id)
}
