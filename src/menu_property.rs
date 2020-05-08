use orbtk::prelude::*;

widget!(MenuProperty<MenuPropertyState> {});

impl Template for MenuProperty {
    fn template(self, _id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("MenuProperty")
            .width(200.)
            .height(400.)
            .margin(Thickness::new(50., 50., 0., 0.))
    }
}

impl MenuProperty {
    pub fn combo_box(items: Vec<String>, selected_index: i32) -> ComboBox {
        let items_len = items.len();

        ComboBox::create()
            .margin((5., 5., 5., 5.))
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
    }
}

#[derive(Default, AsAny)]
pub struct MenuPropertyState {}

impl State for MenuPropertyState {}
