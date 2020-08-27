use amethyst::{
    prelude::*,
    ecs::{Component, DenseVecStorage, Entity},
    core::transform::{Transform, Parent},
    core::math::Vector3,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, SpriteRender, Texture},
    renderer::resources::Tint,
    renderer::palette::Srgba,
    assets::{AssetStorage, Loader, Handle},
};

use crate::slight::ARENA_HEIGHT;

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
    bg_sprite: Handle<SpriteSheet>,
    suit_sprite: Handle<SpriteSheet>,
    number_sprite: Handle<SpriteSheet>,
}

impl Card {
    pub fn new_entity(suit: Suit, number: usize, sprite_symbol: &Handle<SpriteSheet>, sprite_card: &Handle<SpriteSheet>, sprite_number: &Handle<SpriteSheet>, mut transform: Transform, world: &mut World) -> Entity {
        let card = Self {
            suit,
            number,
            flipped: true,
            bg_sprite: sprite_card.clone(),
            suit_sprite: sprite_symbol.clone(),
            number_sprite: sprite_number.clone(),
        };

        const TOP_LEFT: (f32, f32) = (-500.+300., 500.-150.);
        const CARD_HEIGHT: f32 = 1000.;

        const SUIT_WIDTH: f32 = 500.;
        const SUIT_HEIGHT: f32 = 500.;
        const SUIT_SCALE: f32 = 0.4;
        const SUIT_RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
        const SUIT_BLACK: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);

        const FONT_WIDTH: f32 = 40.;
        const FONT_HEIGHT: f32 = 79.;
        const FONT_SCALE: f32 = SUIT_HEIGHT/FONT_HEIGHT;
        const FONT_SHIFT: f32 = SUIT_WIDTH/2. + FONT_WIDTH * FONT_SCALE;

        const CARD_SCALE: f32 = ARENA_HEIGHT/CARD_HEIGHT*0.3;

        transform.set_scale(Vector3::new(CARD_SCALE, CARD_SCALE, CARD_SCALE));


        let suit_sprite = card.suit_sprite();
        let num_sprite = card.number_sprite();
        let tint = match card.suit {
            Suit::Heart | Suit::Diamond => Tint(Srgba::new(SUIT_RED.0, SUIT_RED.1, SUIT_RED.2, SUIT_RED.3)),
            Suit::Club | Suit::Spade => Tint(Srgba::new(SUIT_BLACK.0, SUIT_BLACK.1, SUIT_BLACK.2, SUIT_BLACK.3)),
        };

        let entity = world
            .create_entity()
            .with(card.bg_sprite())
            .with(card)
            .with(transform)
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

    pub fn load_sprites(world: &mut World) -> (Handle<SpriteSheet>, Handle<SpriteSheet>, Handle<SpriteSheet>) {
        let loader = world.read_resource::<Loader>();
        // Suit Sheet
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "suits/suits.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        let suits = loader.load(
            "suits/suits.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        );

        // Card Sheet
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "card/card.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        let card = loader.load(
            "card/card.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        );

        // Number Sheet
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "font/cardtxt.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        let numbers = loader.load(
            "font/cardtxt.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        );

        (suits, card, numbers)
    }

    fn suit_sprite(&self) -> SpriteRender {
        match self.suit {
            Suit::Heart => SpriteRender::new(self.suit_sprite.clone(), 0),
            Suit::Diamond => SpriteRender::new(self.suit_sprite.clone(), 1),
            Suit::Spade => SpriteRender::new(self.suit_sprite.clone(), 2),
            Suit::Club => SpriteRender::new(self.suit_sprite.clone(), 3),
        }
    }

    fn bg_sprite(&self) -> SpriteRender {
        match self.flipped {
            true => SpriteRender::new(self.bg_sprite.clone(), 0),
            false => SpriteRender::new(self.bg_sprite.clone(), 1),
        }
    }

    fn number_sprite(&self) -> SpriteRender {
        assert!(self.number > 0 && self.number < 14, "Invalid card number");
        match self.number {
            1 => SpriteRender::new(self.number_sprite.clone(), 0),
            n if n>1 && n < 14 => SpriteRender::new(self.number_sprite.clone(), n),
            _ => unreachable!()
        }
    }
}

impl Component for Card {
    type Storage = DenseVecStorage<Self>;
}
