use lazy_static::lazy_static;
use rusttype::Font;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
pub struct FontManager {
    db: HashMap<&'static str, Font<'static>>,
    default: Option<Font<'static>>,
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
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        let font = Font::try_from_bytes(static_buffer).expect("Unable to load font.");

        if let Some(name) = name_opt {
            self.db.insert(name, font);
        } else {
            self.default = Some(font);
        }

        Ok(())
    }

    pub fn get_font(&self, key_opt: &Option<String>) -> Option<&Font> {
        if let Some(key) = key_opt {
            self.db.get(key.as_str())
        } else {
            self.default.as_ref()
        }
    }
}