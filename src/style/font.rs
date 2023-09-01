use lazy_static::lazy_static;
use rusttype::Font;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Regular
    }
}

#[derive(Default)]
pub struct FontManager {
    db: HashMap<&'static str, HashMap<FontWeight, Font<'static>>>,
    default: HashMap<FontWeight, Font<'static>>,
}

lazy_static! {
    pub static ref INSTANCE: RwLock<FontManager> = RwLock::new(FontManager::default());
}

impl FontManager {
    pub fn get() -> RwLockReadGuard<'static, FontManager> {
        INSTANCE.read().unwrap()
    }

    pub fn get_mut() -> RwLockWriteGuard<'static, FontManager> {
        INSTANCE.write().unwrap()
    }
    pub fn load(
        &mut self,
        name_opt: Option<&'static str>,
        font_weight: FontWeight,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        let font = Font::try_from_bytes(static_buffer).expect("Unable to load font.");

        if let Some(name) = name_opt {
            let weight_map = self.db.entry(name).or_insert_with(HashMap::new);
            weight_map.insert(font_weight, font);
        } else {
            self.default.insert(font_weight, font);
        }

        Ok(())
    }

    pub fn get_font(&self, key_opt: Option<&String>, font_weight: FontWeight) -> Option<&Font> {
        if let Some(key) = key_opt {
            self.db.get(key.as_str())?.get(&font_weight)
        } else {
            self.default.get(&font_weight)
        }
    }
}
