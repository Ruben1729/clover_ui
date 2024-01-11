#[derive(Debug)]
pub enum FontFamily {
    Style(FontStyle),
    Name(String)
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::Style(Default::default())
    }
}

#[derive(Default, Debug)]
pub enum FontStyle {
    Serif,
    #[default]
    SansSerif,
    Monospace,
    Cursive,
    Fantasy
}
