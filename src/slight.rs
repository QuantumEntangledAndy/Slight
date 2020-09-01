use crate::cache::SpriteCache;
use crate::card::{Card, Suit};

use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{ActiveCamera, Camera},
};

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1024.0;

pub struct Slight;

impl SimpleState for Slight {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SpriteCache::new());

        initialise_camera(world);
        initialise_card(world);
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

/// Initialises one card on the left, and one on the right.
fn initialise_card(world: &mut World) {
    world.register::<Card>();

    let mut card1 = Card::new(Suit::Heart, 1);
    card1.set_floating(100., ARENA_HEIGHT / 2.);
    Card::build(card1, world);

    let mut card2 = Card::new(Suit::Spade, 2);
    card2.set_floating(ARENA_WIDTH - 100., ARENA_HEIGHT / 2.);
    Card::build(card2, world);
}
