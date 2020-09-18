use crate::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::components::card::{Card, Suit};
use crate::utils::mouse_ray;
use crate::components::mousetracking::MouseTracking;
use crate::components::boundingbox::BoundingBox;
use crate::states::splash::SplashScreen;

use amethyst::{
    core::transform::Transform,
    core::math::{Vector2},
    prelude::*,
    input::{InputEvent, Button},
    winit::MouseButton,
    ecs::{Entity, Join},
    assets::{ProgressCounter, Prefab, Handle},
};

use log::*;
use std::collections::HashMap;
use std::mem;


// Load state
pub struct DummyLoad{
    progress: ProgressCounter,
    my_assets: HashMap<String, Entity>,
    game_assets: HashMap<String, Entity>,
}

impl SimpleState for DummyLoad {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.splashy(world);
        self.initialise_card(world);
    }

    #[allow(clippy::type_complexity)]
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;
        // Checks if we are still loading data
        if self.progress.is_complete() {
            debug!("Debug Assets loaded");
            self.un_splashy(world);
            // Transfer game assets we have built to the new game
            return  Trans::Switch(Box::new(Dummy::new_with_assets(mem::replace(&mut self.game_assets, HashMap::new()))));
        }

        Trans::None
    }
}

impl Default for DummyLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl DummyLoad {

    pub fn new() -> Self {
        Self{
            progress: Default::default(),
            my_assets: HashMap::new(),
            game_assets: HashMap::new(),
        }
    }

    /// Initialises one card on the left, and one on the right.
    fn initialise_card(&mut self, world: &mut World) {
        world.register::<Card>();
        world.register::<BoundingBox>();

        let mut card1 = Card::new(Suit::Heart, 1);
        card1.set_floating(100., ARENA_HEIGHT / 2.);
        self.game_assets.insert("TestCard1".to_string(), Card::build(card1, world, &mut self.progress));

        let mut card2 = Card::new(Suit::Spade, 2);
        card2.set_floating(ARENA_WIDTH - 100., ARENA_HEIGHT / 2.);
        self.game_assets.insert("TestCard2".to_string(), Card::build(card2, world, &mut self.progress));
    }


    fn splashy(&mut self, world: &mut World) {
        // Start loading animated splashy
        let splash_prefab = (*world.read_resource::<Handle<Prefab<SplashScreen>>>()).clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 99.0);

        self.my_assets.insert("load_screen".to_string(), world.create_entity()
        .with(splash_prefab)
        .with(transform)
        .build());

        SplashScreen::start_anim(world, self.my_assets.get("load_screen").copied().unwrap());
    }

    fn un_splashy(&mut self, world: &mut World) {
        for ent in self.my_assets.values() {
            let _ = world.delete_entity(*ent);
        }
    }
}


// Play state
#[allow(dead_code)]
pub struct Dummy{
    my_assets: HashMap<String, Entity>,
}

impl SimpleState for Dummy {
    fn handle_event(&mut self, data:StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Input(InputEvent::ButtonPressed(Button::Mouse(MouseButton::Left))) = &event {
            // Clicking
            if let Some(m_ray) = mouse_ray(data.world) {
                let m_pos = m_ray.at_distance(0.);
                let m_pos2 = Vector2::new(m_pos[0], m_pos[1]);
                let entities = data.world.entities();
                let transforms = data.world.read_storage::<Transform>();
                let boxes = data.world.read_storage::<BoundingBox>();
                let closest = (&entities, &transforms, &boxes).join().filter(|(_et, ct, cb)| cb.hit_ww_trans(&ct, &m_pos2)).max_by(|(_, a, _), (_, b, _)| (a.translation()[2]).partial_cmp(&b.translation()[2]).unwrap_or(std::cmp::Ordering::Equal));
                if let Some(mut closest) = closest {
                    let (entity, _, _) = &mut closest;
                    let mut mouse_trackings = data.world.write_storage::<MouseTracking>();
                    if let Some(mt) = mouse_trackings.get_mut(*entity) {
                        mt.activate(&m_pos2);
                    }

                }
            }

        }

        Trans::None
    }
}

impl Default for Dummy {
    fn default() -> Self {
        Self::new()
    }
}

impl Dummy {

    pub fn new() -> Self {
        Self{
            my_assets: HashMap::new(),
        }
    }
    pub fn new_with_assets(assets: HashMap<String, Entity>) -> Self {
        Self{
            my_assets: assets,
        }
    }
}
