use crate::element::ElementType;
use crate::ui::Ui;

impl<'a> Ui<'a> {
    pub fn on_click(&mut self) {
        if let Some(idx) = self.current_interactive_widget {
            println!(
                "{}",
                match self.dom_tree[idx].ty {
                    ElementType::Layout => {
                        "Layout"
                    }
                    ElementType::Label(_) => {
                        "Label"
                    }
                    ElementType::TextEdit(_) => {
                        "TextEdit"
                    }
                }
            );
        }
    }
}
