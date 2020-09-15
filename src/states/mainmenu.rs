use crate::{ARENA_WIDTH, ARENA_HEIGHT};

use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Entity},
    assets::{PrefabLoader, RonFormat, ProgressCounter},
    renderer::{
        sprite::{prefab::SpriteScenePrefab},
    },
};

#[allow(unused_imports)]
use log::*;
use std::collections::HashMap;

pub struct MainMenu {
    my_entities: HashMap<String, Entity>,
    progress: ProgressCounter,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Loading main screen");
        let StateData { world, .. } = data;

        { // BG
            let bg_prefab = world.exec(|loader: PrefabLoader<'_, SpriteScenePrefab>| {
                loader.load(
                    "mainmenu/bg.ron",
                    RonFormat,
                    &mut self.progress,
                )
            });

            let mut transform = Transform::default();
            transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

            self.my_entities.insert("bg".to_string(), world.create_entity()
                .with(bg_prefab)
                .with(transform)
                .build()
            );
        }

        debug!("Menu screen loaded");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        for ent in self.my_entities.values() {
            let _ = world.delete_entity(*ent);
        }
        self.my_entities = HashMap::new();
    }

    fn handle_event(&mut self, _data:StateData<'_, GameData<'_, '_>>, _event: StateEvent) -> SimpleTrans {
        Trans::None
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self{
            my_entities: HashMap::new(),
            progress: Default::default(),
        }
    }
}
