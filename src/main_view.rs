use orbtk::prelude::*;
use orbtk::behaviors::MouseBehavior;

use crate::MainState;

#[derive(Debug)]
pub struct NodeType {
    pub node_type: String16,
    // pub inputs: Vec<u32>,
}

widget!(
    MainView<MainState> {
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            // .nodes(Vec::new())
    }
}
