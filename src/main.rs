use clover_ui::style::{Color, StyleProperty, StyleSheet};
use clover_ui::ui::Ui;

fn main() {
    let mut ui = Ui::default();

    let mut text_ss = StyleSheet::new();
    text_ss.set_backgroundcolor(Color::new(0, 0, 0, 255));

    ui.with_style_sheet(text_ss).flex(|ui| {
        ui.flex(|ui| {
            ui.label("yo momma".to_string());
            ui.label("yo papa".to_string());
            ui.label("you fat".to_string());
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