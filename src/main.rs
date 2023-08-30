use clover_ui::style::{Color, StyleSheet};
use clover_ui::ui::Ui;

fn main() {
    let mut ui = Ui::default();

    let mut text_ss = StyleSheet::new();
    text_ss.set_backgroundcolor(Color::new(0, 0, 0, 255));
    text_ss.set_color(Color::new(0, 0, 255, 255));

    let mut header = StyleSheet::new();
    header.set_backgroundcolor(Color::new(0, 255, 0, 0));
    header.set_color(Color::new(255, 255, 0, 0));

    ui.with_style_sheet(text_ss).flex(|ui| {
        ui.flex(|ui| {
            ui.with_style_sheet(header).label("first".to_string());
            ui.label("second".to_string());
            ui.label("third".to_string());
        });

        ui.flex(|ui| {
            ui.label("other".to_string());
        });
    });

    ui.flex(|ui| {
        ui.label("sibling".to_string());
    });

    ui.traverse_and_print();
}
