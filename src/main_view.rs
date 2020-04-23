use orbtk::prelude::*;
use orbtk::behaviors::MouseBehavior;

use crate::{
    MainState,
    node_view::NodeView,
};

#[derive(Debug)]
pub struct NodeType {
    pub node_type: String16,
    pub inputs: Vec<u32>,
}

// into_property_source!(NodeType);

type NodeTypes = Vec<NodeType>;

widget!(
    MainView<MainState> {
        node_types: NodeTypes
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(
                NodeView::create()
                .title("hehe")
                .build(ctx),
            )
    }
}
