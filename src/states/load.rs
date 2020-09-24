use crate::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::cache::{SpriteCache, FontCache, SpriteScenePrefabCache};
pub(crate) use crate::components::loadscreen::LoadScreen;
pub(crate) use crate::components::loadscreen::AnimationId;
use crate::components::clickable::Clickable;
use crate::components::droppoint::DropPoint;
use super::mainmenu::MainMenu;

use amethyst::{
    core::transform::Transform,
    prelude::*,
    ecs::Entity,
    renderer::{ActiveCamera, Camera},
    assets::{ProgressCounter, Prefab, PrefabLoader, RonFormat, Handle},
    renderer::{
        sprite::{prefab::SpriteScenePrefab},
    },
};

use log::*;

pub struct Load{
    progress_counter: Option<ProgressCounter>,
    assets_counter: Option<ProgressCounter>,
    splahing: bool,
    load_ent: Option<Entity>,
}

impl SimpleState for Load {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SpriteCache::new());
        world.insert(FontCache::new());
        world.insert(SpriteScenePrefabCache::new());

        self.progress_counter = Some(Default::default());
        let load_prefab = world.exec(|loader: PrefabLoader<'_, LoadScreen>| {
            loader.load(
                "load/loadscreen.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap(),
            )
        });
        world.insert(load_prefab);
        world.register::<Clickable>();
        world.register::<DropPoint>();

        Self::initialise_camera(world);
        self.loady(world);
        self.load_assets(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.un_loady(world);
    }

    #[allow(clippy::type_complexity)]
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if ! self.splahing {
            if let Some(ref progress_counter) = self.progress_counter {
                if progress_counter.is_complete() {
                    debug!("Loaded sprite");
                    let StateData { world, .. } = data;
                    let loading_ent = self.load_ent;
                    LoadScreen::start_anim_soon(world, loading_ent.unwrap());
                    // All data loaded
                    self.progress_counter = None;
                    debug!("Loaded anims");
                    self.splahing = true;
                }
            }
        } else if let Some(ref assets_counter) = self.assets_counter {
            if assets_counter.is_complete() {
                debug!("Assets loaded");
                return  Trans::Replace(Box::new(MainMenu::new()));
            }
        }

        Trans::None
    }
}

impl Default for Load {
    fn default() -> Self {
        Self::new()
    }
}

impl Load {
    pub fn new() -> Self {
        Self{
            progress_counter: None,
            assets_counter: None,
            splahing: false,
            load_ent: None,
        }
    }

    fn initialise_camera(world: &mut World) {
        // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 100.0);

        let cam_ent = world
            .create_entity()
            .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
            .with(transform)
            .build();

        let act_cam: &mut ActiveCamera = world.get_mut().expect("There shoud be an active camera");
        act_cam.entity = Some(cam_ent);
    }

    fn loady(&mut self, world: &mut World) {
        // Start loading animated loady
        let load_prefab = (*world.read_resource::<Handle<Prefab<LoadScreen>>>()).clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 99.0);

        self.load_ent = Some(world.create_entity()
        .with(load_prefab)
        .with(transform)
        .build());
    }

    fn un_loady(&mut self, world: &mut World) {
        // Stop the loady screen
        if let Some(load_ent) = self.load_ent {
            let _ = world.delete_entity(load_ent);
            self.load_ent = None;
        }
    }

    fn load_assets(&mut self, world: &mut World) {
        // This should load assets and add them to the assets_counter
        self.assets_counter = Some(Default::default());
        {
            let mut cache = world.fetch_mut::<FontCache>();
            cache.get_or_insert_progress("font/square", world, self.assets_counter.as_mut().unwrap());
        }
        {
            let mm_prefab = {
                world.exec(|loader: PrefabLoader<'_, SpriteScenePrefab>| {
                    loader.load(
                        "mainmenu/bg.ron",
                        RonFormat,
                        self.assets_counter.as_mut().unwrap(),
                    )
                })
            };
            let mut cache = world.fetch_mut::<SpriteScenePrefabCache>();
            cache.insert("mainmenu/bg", mm_prefab);
        }
    }
}
