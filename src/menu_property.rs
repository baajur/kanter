use orbtk::prelude::*;

widget!(
    MenuProperty<MenuPropertyState> {
        open: bool
    }
);

impl Template for MenuProperty {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MenuProperty")
            .width(200.)
            .height(400.)
            .open(false)
            .margin(Thickness::new(50., 50., 0., 0.))
            .child(
                Container::create()
                    .background("#aa0000")
                    .child(
                        Stack::create()
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}

impl MenuProperty {
    pub fn combo_box(ctx: &mut Context, parent: Entity, items: Vec<String>, selected_index: i32) {
        let items_len = items.len();

        let bc = &mut ctx.build_context();
        let combo_box = ComboBox::create()
            .items_builder(move |bc, index| {
                let text = &items[index];
                TextBlock::create()
                    .margin((0.0, 0.0, 0.0, 2.0))
                    .vertical_alignment("start")
                    .text(text.as_str())
                    .build(bc)
            })
            .count(items_len)
            .selected_index(selected_index)
            .build(bc);

        bc.append_child(parent, combo_box);
    }
}

#[derive(Default, AsAny)]
pub struct MenuPropertyState {
    height: f64,
}

impl State for MenuPropertyState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.height = ctx.widget().get::<Rectangle>("bounds").height();
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(open) = ctx.widget().try_get::<bool>("open") {
            if *open {
                ctx.widget().get_mut::<Rectangle>("bounds").set_height(self.height);
            } else {
                ctx.widget().get_mut::<Rectangle>("bounds").set_height(0.);
            }
        }
    }
}
