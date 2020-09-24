use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, Prefab},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontAsset, TtfFormat},
    renderer::{
        sprite::{prefab::SpriteScenePrefab},
    },
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
    #[allow(dead_code)]
    pub fn get_or_insert(&mut self, key: &str, world: &World) -> Handle<SpriteSheet> {
        self.get_or_insert_progress(key, world, &mut Default::default())
    }

    pub fn get_or_insert_progress(&mut self, key: &str, world: &World, progress: &mut ProgressCounter) -> Handle<SpriteSheet> {
        if let Some(value) = self.map.get(key) {
            value.clone()
        } else {
            // Suit Sheet
            let texture_handle = {
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                world.read_resource::<Loader>().load(
                    format!("{}.png", key),
                    ImageFormat::default(),
                    progress,
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
        self.get_or_insert_progress(key, world, &mut Default::default())
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


pub struct SpriteScenePrefabCache {
    map: HashMap<String, Handle<Prefab<SpriteScenePrefab>>>,
}

impl SpriteScenePrefabCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: &str, value: Handle<Prefab<SpriteScenePrefab>>) {
        self.map.insert(key.to_string(), value.clone());
    }

    pub fn get(&self, key: &str) -> Handle<Prefab<SpriteScenePrefab>> {
        self.map.get(key).unwrap().clone()
    }
}
