use crate::{node_view::NODE_SIZE, shared::*, slot_state::SlotState};
use orbtk::{behaviors::MouseBehavior, prelude::*};

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

pub const SLOT_SIZE: f64 = 15.;
pub const SLOT_SIZE_HALF: f64 = SLOT_SIZE * 0.5;
pub const SLOT_SPACING: f64 = SLOT_SIZE_HALF;

impl Template for SlotView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let margin_left = match self.side.as_ref().unwrap() {
            PropertySource::Value(Side::Input) => -SLOT_SIZE_HALF,
            PropertySource::Value(Side::Output) => NODE_SIZE - SLOT_SIZE_HALF,
            _ => {
                panic!("Side is not properly set");
            }
        };

        let margin = Thickness {
            left: margin_left,
            right: 0.,
            top: 0.,
            bottom: SLOT_SPACING,
        };

        self.name("SlotView")
            .width(SLOT_SIZE)
            .height(SLOT_SIZE)
            .margin(margin)
            .on_mouse_down(move |states, _| {
                states
                    .get_mut::<SlotState>(id)
                    .mouse_action(MouseAction::MousePressed);
                false
            })
            .on_mouse_up(move |states, _| {
                states
                    .get_mut::<SlotState>(id)
                    .mouse_action(MouseAction::MouseReleased);
                false
            })
            .child(MouseBehavior::create().enabled(id).target(id.0).build(ctx))
            .child(
                Container::create()
                    .background(Color::rgb(200, 200, 200))
                    .border_width(1.)
                    .border_radius(SLOT_SIZE_HALF)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
                    .build(ctx),
            )
    }
}
