use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
}

impl State for NodeState {
}