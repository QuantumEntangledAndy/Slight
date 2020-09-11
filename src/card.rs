use crate::cache::SpriteCache;
use crate::mousetracking::MouseTracking;
use crate::boundingbox::BoundingBox;
use crate::slight::ARENA_HEIGHT;

use amethyst::{
    assets::Handle,
    core::math::Vector3,
    core::transform::{Parent, Transform},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::palette::Srgba,
    renderer::resources::Tint,
    renderer::{SpriteRender, SpriteSheet},
};

#[allow(dead_code)]
enum Location {
    Hand,
    Deck,
    Pile,
    Floating(f32, f32),
}

#[allow(dead_code)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

pub struct Card {
    suit: Suit,
    number: usize,
    flipped: bool,
    location: Location,
}

impl Card {
    pub fn new(suit: Suit, number: usize) -> Self {
        Self {
            suit,
            number,
            flipped: true,
            location: Location::Floating(0., 0.),
        }
    }

    pub fn set_floating(&mut self, x: f32, y: f32) {
        self.location = Location::Floating(x, y);
    }

    pub fn build(card: Self, world: &mut World) -> Entity {
        let mut transform: Transform = Transform::default();

        const TOP_LEFT: (f32, f32) = (-500. + 300., 500. - 150.);
        const CARD_WIDTH: f32 = 500.;
        const CARD_HEIGHT: f32 = 1000.;

        const SUIT_WIDTH: f32 = 500.;
        const SUIT_HEIGHT: f32 = 500.;
        const SUIT_SCALE: f32 = 0.4;
        const SUIT_RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
        const SUIT_BLACK: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);

        const FONT_WIDTH: f32 = 40.;
        const FONT_HEIGHT: f32 = 79.;
        const FONT_SCALE: f32 = SUIT_HEIGHT / FONT_HEIGHT;
        const FONT_SHIFT: f32 = SUIT_WIDTH / 2. + FONT_WIDTH * FONT_SCALE;

        const CARD_SCALE: f32 = ARENA_HEIGHT / CARD_HEIGHT * 0.3;

        transform.set_scale(Vector3::new(CARD_SCALE, CARD_SCALE, CARD_SCALE));

        let mouse_tracking = MouseTracking::new();
        let boundingbox = BoundingBox::new(CARD_WIDTH, CARD_HEIGHT);

        if let Location::Floating(x, y) = card.location {
            transform.set_translation_xyz(x, y, 0.0);
            //mouse_tracking.activate_xy(x, y);
        }

        let suit_sprite = card.suit_sprite(world);
        let num_sprite = card.number_sprite(world);
        let bg_sprite = card.bg_sprite(world);
        let tint = match card.suit {
            Suit::Heart | Suit::Diamond => {
                Tint(Srgba::new(SUIT_RED.0, SUIT_RED.1, SUIT_RED.2, SUIT_RED.3))
            }
            Suit::Club | Suit::Spade => Tint(Srgba::new(
                SUIT_BLACK.0,
                SUIT_BLACK.1,
                SUIT_BLACK.2,
                SUIT_BLACK.3,
            )),
        };

        let entity = world
            .create_entity()
            .with(bg_sprite)
            .with(card)
            .with(boundingbox)
            .with(transform)
            .with(mouse_tracking)
            .build();

        // Top left
        let mut suit_transform = Transform::default();
        suit_transform.set_translation_xyz(TOP_LEFT.0, TOP_LEFT.1, 1.);
        suit_transform.set_scale(Vector3::new(SUIT_SCALE, SUIT_SCALE, SUIT_SCALE));

        let suit = world
            .create_entity()
            .with(suit_sprite.clone())
            .with(suit_transform)
            .with(Parent { entity })
            .with(tint)
            .build();

        let mut text_trans = Transform::default();
        text_trans.set_translation_xyz(FONT_SHIFT, 0., 1.);
        text_trans.set_scale(Vector3::new(FONT_SCALE, FONT_SCALE, FONT_SCALE));
        world
            .create_entity()
            .with(num_sprite.clone())
            .with(text_trans)
            .with(Parent { entity: suit })
            .with(tint)
            .build();

        // Bottom Right
        let mut suit_transform = Transform::default();
        suit_transform.set_translation_xyz(-TOP_LEFT.0, -TOP_LEFT.1, 1.);
        suit_transform.set_scale(Vector3::new(-SUIT_SCALE, -SUIT_SCALE, SUIT_SCALE));

        let suit = world
            .create_entity()
            .with(suit_sprite)
            .with(suit_transform)
            .with(Parent { entity })
            .with(tint)
            .build();

        let mut text_trans = Transform::default();
        text_trans.set_translation_xyz(FONT_SHIFT, 0., 1.);
        text_trans.set_scale(Vector3::new(FONT_SCALE, FONT_SCALE, FONT_SCALE));
        world
            .create_entity()
            .with(num_sprite)
            .with(text_trans)
            .with(Parent { entity: suit })
            .with(tint)
            .build();

        entity
    }

    fn load_suit_sprite(world: &World) -> Handle<SpriteSheet> {
        let mut cache = world.fetch_mut::<SpriteCache>();
        cache.get_or_insert("suits/suits", world)
    }

    fn load_card_sprite(world: &World) -> Handle<SpriteSheet> {
        let mut cache = world.fetch_mut::<SpriteCache>();
        cache.get_or_insert("card/card", world)
    }

    fn load_number_sprite(world: &World) -> Handle<SpriteSheet> {
        let mut cache = world.fetch_mut::<SpriteCache>();
        cache.get_or_insert("font/cardtxt", world)
    }

    fn suit_sprite(&self, world: &World) -> SpriteRender {
        let suit_sprite = Self::load_suit_sprite(world);
        match self.suit {
            Suit::Heart => SpriteRender::new(suit_sprite, 0),
            Suit::Diamond => SpriteRender::new(suit_sprite, 1),
            Suit::Spade => SpriteRender::new(suit_sprite, 2),
            Suit::Club => SpriteRender::new(suit_sprite, 3),
        }
    }

    fn bg_sprite(&self, world: &World) -> SpriteRender {
        let bg_sprite = Self::load_card_sprite(world);
        match self.flipped {
            true => SpriteRender::new(bg_sprite, 0),
            false => SpriteRender::new(bg_sprite, 1),
        }
    }

    fn number_sprite(&self, world: &World) -> SpriteRender {
        assert!(self.number > 0 && self.number < 14, "Invalid card number");
        let number_sprite = Self::load_number_sprite(world);
        match self.number {
            1 => SpriteRender::new(number_sprite, 0),
            n if n > 1 && n < 14 => SpriteRender::new(number_sprite, n),
            _ => unreachable!(),
        }
    }
}

impl Component for Card {
    type Storage = DenseVecStorage<Self>;
}
