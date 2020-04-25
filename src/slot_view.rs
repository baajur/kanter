use crate::node_state::{Action, NodeState};
use orbtk::{behaviors::MouseBehavior, prelude::*};

#[derive(Copy, Clone, Debug)]
pub enum SlotType {
    Input,
    Output,
}
into_property_source!(SlotType);

impl Default for SlotType {
    fn default() -> Self {
        Self::Input
    }
}

widget!(
    SlotView<NodeState>: MouseHandler {
        slot_type: SlotType,
        node_workspace: Entity,
        slot_id: String16
    }
);

impl Template for SlotView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let size = 15.;

        let margin = Thickness {
            left: -(size*0.5),
            right: 0.,
            top: 0.,
            bottom: size*0.5,
        };

        self.name("SlotView")
            .width(size)
            .height(size)
            .margin(margin)
            .child(
                Container::create()
                    .background(Color::rgb(255, 255, 0))
                    .border_width(1.)
                    .border_radius(size*0.49)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
                    .build(ctx),
            )
    }
}
