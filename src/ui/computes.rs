use crate::element::{Element, ElementType};
use crate::style::{FontManager};
use crate::ui::Ui;
use rusttype::{point, Scale};

impl Ui {
    pub fn compute_text_dimensions(&self, element: &mut Element) {
        if let ElementType::Label(value) = element.ty {
            let manager = FontManager::get();
            let font = manager
                .get_font(&element.style.get_fontfamily())
                .expect("Unable to load font");

            let scale = Scale::uniform(element.style.get_fontsize());
            let v_metrics = font.v_metrics(scale);

            let glyphs: Vec<_> = font
                .layout(value, scale, point(0.0, 0.0 + v_metrics.ascent))
                .collect();

            let (content_w, content_h) = (
                {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    (max_x - min_x) as usize
                },
                (v_metrics.ascent - v_metrics.descent).ceil() as usize,
            );

            element.style.set_width(content_w);
            element.style.set_height(content_h);
        } else {
            panic!("Attempted to call compute content dimensions on a non label.");
        }
    }
}
