use crate::element::ElementType;
use crate::style::{FlexDirection, FontManager, Layout};
use crate::ui::Ui;
use rusttype::{point, Scale};

impl<'a> Ui<'a> {
    pub fn compute_dimensions(&mut self) {
        self._compute_dimensions(self.root_idx);
    }
    fn _compute_dimensions(&mut self, element_idx: usize) {
        let children_ids = self.dom_tree[element_idx].children.clone();
        for child_idx in children_ids {
            self._compute_dimensions(child_idx);
        }

        let mut content_w = 0;
        let mut content_h = 0;
        match &self.dom_tree[element_idx].ty {
            ElementType::Layout => {
                (content_w, content_h) = match &self.dom_tree[element_idx].style.get_display() {
                    Layout::Block => (
                        self.dom_tree[element_idx]
                            .children
                            .iter()
                            .map(|child_idx| self.dom_tree[*child_idx].style.get_total_width())
                            .max()
                            .unwrap_or_default(),
                        self.dom_tree[element_idx]
                            .children
                            .iter()
                            .map(|child_idx| self.dom_tree[*child_idx].style.get_total_height())
                            .sum(),
                    ),
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row | FlexDirection::RowReverse => (
                            self.dom_tree[element_idx]
                                .children
                                .iter()
                                .map(|child_idx| self.dom_tree[*child_idx].style.get_total_width())
                                .sum(),
                            self.dom_tree[element_idx]
                                .children
                                .iter()
                                .map(|child_idx| self.dom_tree[*child_idx].style.get_total_height())
                                .max()
                                .unwrap_or_default(),
                        ),
                        FlexDirection::Col | FlexDirection::ColReverse => (
                            self.dom_tree[element_idx]
                                .children
                                .iter()
                                .map(|child_idx| self.dom_tree[*child_idx].style.get_total_width())
                                .max()
                                .unwrap_or_default(),
                            self.dom_tree[element_idx]
                                .children
                                .iter()
                                .map(|child_idx| self.dom_tree[*child_idx].style.get_total_height())
                                .sum(),
                        ),
                    },
                    Layout::Grid { .. } | Layout::InlineBlock | Layout::Hidden => (0, 0),
                };
            }
            ElementType::Label(value) => {
                let manager = FontManager::get();
                let font = manager
                    .get_font(&self.dom_tree[element_idx].style.get_fontfamily())
                    .expect("Unable to load font");

                let scale = Scale::uniform(self.dom_tree[element_idx].style.get_fontsize());
                let v_metrics = font.v_metrics(scale);

                let glyphs: Vec<_> = font
                    .layout(value, scale, point(0.0, 0.0 + v_metrics.ascent))
                    .collect();

                (content_w, content_h) = (
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
                )
            }
            _ => {}
        };

        self.dom_tree[element_idx].style.set_width(content_w);
        self.dom_tree[element_idx].style.set_height(content_h);
    }

    pub fn compute_positions(&mut self) {
        self._compute_positions(self.root_idx, 0, 0);
    }

    pub fn _compute_positions(&mut self, element_idx: usize, dx: usize, dy: usize) {
        {
            self.dom_tree[element_idx].style.set_x(dx);
            self.dom_tree[element_idx].style.set_y(dy);
        }

        let (mut new_dx, mut new_dy) = match self.dom_tree[element_idx].style.get_display() {
            Layout::Block => (
                self.dom_tree[element_idx].style.get_content_x(),
                self.dom_tree[element_idx].style.get_content_y(),
            ),
            Layout::Flex { flex_direction, .. } => match flex_direction {
                FlexDirection::Row | FlexDirection::Col => (
                    self.dom_tree[element_idx].style.get_content_x(),
                    self.dom_tree[element_idx].style.get_content_y(),
                ),
                FlexDirection::RowReverse => (
                    self.dom_tree[element_idx].style.get_content_x()
                        + self.dom_tree[element_idx].style.get_width(),
                    self.dom_tree[element_idx].style.get_content_y(),
                ),
                FlexDirection::ColReverse => (
                    self.dom_tree[element_idx].style.get_content_x(),
                    self.dom_tree[element_idx].style.get_content_y()
                        + self.dom_tree[element_idx].style.get_height(),
                ),
            },
            _ => (0, 0),
        };

        let child_ids = self.dom_tree[element_idx].children.clone();

        for child_idx in child_ids {
            {
                match &self.dom_tree[element_idx].style.get_display() {
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row | FlexDirection::Col => {}
                        FlexDirection::RowReverse => {
                            new_dx -= self.dom_tree[child_idx].style.get_total_width();
                        }
                        FlexDirection::ColReverse => {
                            new_dy -= self.dom_tree[child_idx].style.get_total_height();
                        }
                    },
                    _ => {}
                }
            }
            self._compute_positions(child_idx, new_dx, new_dy);
            {
                match &self.dom_tree[element_idx].style.get_display() {
                    Layout::Block => {
                        new_dy += self.dom_tree[child_idx].style.get_total_height();
                    }
                    Layout::Flex { flex_direction, .. } => match flex_direction {
                        FlexDirection::Row => {
                            new_dx += self.dom_tree[child_idx].style.get_total_width();
                        }
                        FlexDirection::RowReverse | FlexDirection::ColReverse => {}
                        FlexDirection::Col => {
                            new_dy += self.dom_tree[child_idx].style.get_total_height();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}
