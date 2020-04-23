use orbtk::prelude::*;

use crate::{
    MainState,
    node_container_view::NodeContainerView,
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
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(
                NodeContainerView::create()
                .build(ctx),
            )
    }
}
