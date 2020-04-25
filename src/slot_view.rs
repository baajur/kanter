use crate::{
    node_view::NODE_SIZE,
    slot_state::SlotState,
};
use orbtk::{prelude::*};

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Input,
    Output,
}
into_property_source!(Side);

impl Default for Side {
    fn default() -> Self {
        Self::Input
    }
}

widget!(
    SlotView<SlotState>: MouseHandler {
        side: Side,
        node_workspace: Entity,
        slot_id: String16
    }
);

impl Template for SlotView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let size = 15.;

        let margin_left = match (&self).side.as_ref().unwrap() {
            PropertySource::Value(Side::Input) => -(size*0.5),
            PropertySource::Value(Side::Output) => NODE_SIZE - (size*0.5),
            _ => {
                panic!("Side is not properly set");
            }
        };

        let margin = Thickness {
            left: margin_left,
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
                    .background(Color::rgb(200, 200, 200))
                    .border_width(1.)
                    .border_radius(size*0.49)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
                    .build(ctx),
            )
    }
}
