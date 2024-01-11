use lazy_static::lazy_static;
use rusttype::Font;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use super::font_weight::FontWeight;

#[derive(Default)]
pub struct FontDb {
    name_map: HashMap<&'static str, HashMap<FontWeight, Font<'static>>>,
}

lazy_static! {
    pub static ref INSTANCE: RwLock<FontDb> = RwLock::new(FontDb::default());
}

impl FontDb {
    pub fn get() -> RwLockReadGuard<'static, FontDb> {
        INSTANCE.read().unwrap()
    }

    pub fn get_mut() -> RwLockWriteGuard<'static, FontDb> {
        INSTANCE.write().unwrap()
    }
    pub fn load(
        &mut self,
        name: &'static str,
        font_weight: FontWeight,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        let font = Font::try_from_bytes(static_buffer).expect("Unable to load font.");

        let weight_map = self.name_map.entry(name).or_insert_with(HashMap::new);
        weight_map.insert(font_weight, font);

        Ok(())
    }

    pub fn get_font(&self, name: &String, font_weight: FontWeight) -> Option<&Font> {
        self.name_map.get(name.as_str())?.get(&font_weight)
    }
}
