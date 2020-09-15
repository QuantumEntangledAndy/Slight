use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
};
use std::collections::HashMap;

pub struct SpriteCache {
    map: HashMap<String, Handle<SpriteSheet>>,
}

impl SpriteCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get_or_insert(&mut self, key: &str, world: &World) -> Handle<SpriteSheet> {
        if let Some(value) = self.map.get(key) {
            value.clone()
        } else {
            // Suit Sheet
            let texture_handle = {
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                world.read_resource::<Loader>().load(
                    format!("{}.png", key),
                    ImageFormat::default(),
                    (),
                    &texture_storage,
                )
            };
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            let sheet = world.read_resource::<Loader>().load(
                format!("{}.ron", key), // Here we load the associated ron file
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            );
            self.map.insert(key.to_string(), sheet.clone());
            sheet
        }
    }
}

pub struct FontCache {
    map: HashMap<String, Handle<FontAsset>>,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get_or_insert(&mut self, key: &str, world: &World) -> Handle<FontAsset> {
        if let Some(value) = self.map.get(key) {
            value.clone()
        } else {
            let font_handle = {
                let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
                world.read_resource::<Loader>().load(
                    format!("{}.ttf", key),
                    TtfFormat,
                    (),
                    &font_storage,
                )
            };
            self.map.insert(key.to_string(), font_handle.clone());
            font_handle
        }
    }

    pub fn get_or_insert_progress(&mut self, key: &str, world: &World, progress: &mut ProgressCounter) -> Handle<FontAsset> {
        if let Some(value) = self.map.get(key) {
            value.clone()
        } else {
            let font_handle = {
                let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
                world.read_resource::<Loader>().load(
                    format!("{}.ttf", key),
                    TtfFormat,
                    progress,
                    &font_storage,
                )
            };
            self.map.insert(key.to_string(), font_handle.clone());
            font_handle
        }
    }
}
