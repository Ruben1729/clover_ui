use clover_ui::core::{AppBuilder};
use clover_ui::nodes::{BaseNode, TextNode};
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
                let mut base_node = BaseNode::new(
                    Style {
                        content: Bounds::px(30, 30),
                        background_color: COLOR_RED_700,
                        padding: Spacing::uniform(10),
                        margin: Spacing::new(Unit::Auto, Unit::Px(5), Unit::Auto, Unit::Auto),
                        border_color: COLOR_EMERALD_700,
                        border: Spacing::uniform(10),
                        ..Default::default()
                    },
                    |children|{
                        children.push(TextNode::new("Hello World".to_string(),
                        Style {
                            text_color: COLOR_PINK_900,
                            font_size: Unit::Px(30),
                            line_height: Unit::Px(42),
                            content: Bounds {
                                width: Unit::Px(600),
                                height: Unit::Px(160)
                            },
                            .. Default::default()
                        }))
                    }
                );

                children.push(base_node.build());
                children.push(BaseNode::new(
                    Style {
                        content: Bounds::px(100, 100),
                        margin: Spacing::uniform(10),
                        background_color: COLOR_BLUE_700,
                        ..Default::default()
                    },
                    |_|{}
                ).build());
                children.push(BaseNode::new(
                    Style {
                        content: Bounds::px(10, 10),
                        background_color: COLOR_GREEN_700,
                        ..Default::default()
                    },
                    |_|{}
                ).build());
            }).build());
        }).start();
}
