use clover_ui::core::{AppBuilder};
use clover_ui::nodes::BaseNode;
use clover_ui::styles::{Bounds, Spacing, Style, Unit};
use clover_ui::styles::preset::color::*;

extern crate clover_ui;

fn main() {
    AppBuilder::new("test".to_string())
        .build(|children| {
            children.push(BaseNode::new(
                Style {
                    border_color: COLOR_FUCHSIA_900,
                    ..Default::default()
                }
            , |children| {
                children.push(BaseNode::new(
                    Style {
                        content: Bounds::px(30, 30),
                        background_color: COLOR_RED_700,
                        padding: Spacing::uniform(10),
                        margin: Spacing::new(Unit::Auto, Unit::Px(5), Unit::Auto, Unit::Auto),
                        border_color: COLOR_EMERALD_700,
                        border: Spacing::uniform(10),
                        ..Default::default()
                    },
                    |_|{}
                ));
                children.push(BaseNode::new(
                    Style {
                        content: Bounds::px(100, 100),
                        margin: Spacing::uniform(10),
                        background_color: COLOR_BLUE_700,
                        ..Default::default()
                    },
                    |_|{}
                ));
                children.push(BaseNode::new(
                    Style {
                        content: Bounds::px(10, 10),
                        background_color: COLOR_GREEN_700,
                        ..Default::default()
                    },
                    |_|{}
                ));
            }));
        }).start();
}
