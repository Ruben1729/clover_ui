use clover_ui::style::{StyleProperty, StyleSheet};
use clover_ui::ui::Ui;

fn main() {
    let mut ui = Ui::default();

    let mut text_ss = StyleSheet::default();
    text_ss.set_background_color(255, 255, 255, 255);

    ui.with_style_sheet(text_ss).flex(|ui| {
        ui.flex(|ui| {
            ui.label("sub_child1".to_string());
            ui.label("sub_child2".to_string());
            ui.label("sub_child3".to_string());
        });

        ui.flex(|ui| {
            ui.label("hello".to_string());
        });
    });

    ui.flex(|ui| {
        ui.label("sub_child1".to_string());
    });

    ui.traverse_and_print();
}