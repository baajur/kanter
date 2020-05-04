use crate::shared::*;
use orbtk::{behaviors::MouseBehavior, prelude::*};

const SELECTED_BRUSH: Brush = Brush::SolidColor(Color::rgb(255, 255, 255));
const DESELECTED_BRUSH: Brush = Brush::SolidColor(Color::rgb(0, 0, 0));

widget!(
    Node<NodeState> {
        widget_type: WidgetType,
        title: String16,
        my_margin: Thickness,
        node_id: u32,
        slot_count_input: usize,
        slot_count_output: usize,
        selected: bool
    }
);

impl Template for Node {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let property_stack = Stack::create().build(ctx);
        self.state_mut().property_stack = property_stack;

        let frame = Container::create()
            .background(Color::rgb(0, 255, 0))
            .border_width(2.)
            .border_brush(DESELECTED_BRUSH)
            .child(
                Stack::create()
                    .child(
                        TextBlock::create()
                            .id("title")
                            .text(("title", id))
                            .element("text-block")
                            .horizontal_alignment("center")
                            .foreground(Color::rgb(255, 0, 0))
                            .width(0.)
                            .height(14.)
                            .build(ctx),
                    )
                    .child(property_stack)
                    .build(ctx),
            )
            .build(ctx);
        self.state_mut().frame = frame;

        self.name("Node")
            .widget_type(WidgetType::Node)
            .width(NODE_SIZE)
            .height(NODE_SIZE)
            .margin(("my_margin", id))
            .child(MouseBehavior::create().enabled(id).target(id.0).build(ctx))
            .child(frame)
    }
}

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pub builder: WidgetBuildContext,
    frame: Entity,
    property_stack: Entity,
}

impl State for NodeState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let bc = &mut ctx.build_context();

        let property = ComboBox::create()
            .margin(Thickness {
                left: 20.,
                top: 0.,
                right: 0.,
                bottom: 0.,
            })
            .build(bc);

        bc.append_child(self.property_stack, property);
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if *ctx.widget().get::<bool>("selected") {
            ctx.get_widget(self.frame)
                .set::<Brush>("border_brush", SELECTED_BRUSH);
            ctx.get_widget(self.property_stack)
                .set::<bool>("enabled", false);
        } else {
            ctx.get_widget(self.frame)
                .set::<Brush>("border_brush", DESELECTED_BRUSH);
            ctx.get_widget(self.property_stack)
                .set::<bool>("enabled", true);
        }
    }
}
