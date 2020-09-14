mod splashscreen;

use crate::cache::SpriteCache;
pub use self::splashscreen::SplashScreen;
pub use self::splashscreen::AnimationId;
use crate::slight::Slight;

use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
        EndControl,
    },
    ecs::{Entity, Entities, Join, ReadStorage, WriteStorage},
    core::transform::Transform,
    prelude::*,
    renderer::{ActiveCamera, Camera, sprite::SpriteRender,},
    assets::{ProgressCounter, PrefabLoader, RonFormat},
};

use log::*;

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1024.0;

pub struct Splash{
    progress_counter: Option<ProgressCounter>,
    assets_counter: Option<ProgressCounter>,
    splahing: bool,
    splash_ent: Option<Entity>,
}

impl SimpleState for Splash {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SpriteCache::new());

        Self::initialise_camera(world);
        self.splashy(world);
        self.load_assets(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.un_splashy(world);
    }

    #[allow(clippy::type_complexity)]
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if ! self.splahing {
            if let Some(ref progress_counter) = self.progress_counter {
                if progress_counter.is_complete() {
                    debug!("Loaded sprite");
                    let StateData { world, .. } = data;
                    world.exec(
                        |(entities, animation_sets, mut control_sets): (
                            Entities,
                            ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                            WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                        )| {
                            // For each entity that has AnimationSet
                            for (entity, animation_set) in (&entities, &animation_sets).join() {
                                // Creates a new AnimationControlSet for the entity
                                let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                                // Adds the `Fly` animation to AnimationControlSet and loops infinitely
                                control_set.add_animation(
                                    AnimationId::Splash,
                                    &animation_set.get(&AnimationId::Splash).unwrap(),
                                    EndControl::Loop(None),
                                    1.0,
                                    AnimationCommand::Start,
                                );
                            }
                        },
                    );
                    // All data loaded
                    self.progress_counter = None;
                    debug!("Loaded anims");
                    self.splahing = true;
                }
            }
        } else if let Some(ref assets_counter) = self.assets_counter {
            if assets_counter.is_complete() {
                debug!("Assets loaded");
                return  Trans::Switch(Box::new(Slight));
            }
        }

        Trans::None
    }
}

impl Default for Splash {
    fn default() -> Self {
        Self::new()
    }
}

impl Splash {
    pub fn new() -> Self {
        Self{
            progress_counter: None,
            assets_counter: None,
            splahing: false,
            splash_ent: None,
        }
    }

    fn initialise_camera(world: &mut World) {
        // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

        let cam_ent = world
            .create_entity()
            .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
            .with(transform)
            .build();

        let act_cam: &mut ActiveCamera = world.get_mut().expect("There shoud be an active camera");
        act_cam.entity = Some(cam_ent);
    }

    fn splashy(&mut self, world: &mut World) {
        // Start loading animated splashy
        self.progress_counter = Some(Default::default());
        let splash_prefab = world.exec(|loader: PrefabLoader<'_, SplashScreen>| {
            loader.load(
                "splash/splashscreen.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap(),
            )
        });

        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

        self.splash_ent = Some(world.create_entity()
        .with(splash_prefab)
        .with(transform)
        .build());
    }

    fn un_splashy(&mut self, world: &mut World) {
        // Stop the splashy screen
        if let Some(splash_ent) = self.splash_ent {
            let _ = world.delete_entity(splash_ent);
            self.splash_ent = None;
        }
    }

    fn load_assets(&mut self, _world: &mut World) {
        // This should load assets and add them to the assets_counter
        self.assets_counter = Some(Default::default());
    }
}
