use crate::card::{Card, Suit};

use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::{Camera},
};

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1024.0;

pub struct Slight;

impl SimpleState for Slight {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        initialise_card(world);
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

/// Initialises one card on the left, and one on the right.
fn initialise_card(world: &mut World) {
    world.register::<Card>();
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the cards.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(0., y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH/2., y, 0.0);

    let (symbol_sheet, card_sheet, number_sheet) = Card::load_sprites(world);

    Card::new_entity(Suit::Heart, 1, &symbol_sheet, &card_sheet, &number_sheet, left_transform, world);

    Card::new_entity(Suit::Spade, 2, &symbol_sheet, &card_sheet, &number_sheet, right_transform, world);
}
