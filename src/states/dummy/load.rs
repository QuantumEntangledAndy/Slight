use super::play::Dummy;

use crate::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::components::card::{Card, Suit};
use crate::components::clickable::{Clickable, ClickAction};
use crate::components::boundingbox::BoundingBox;
use crate::states::load::LoadScreen;

use amethyst::{
    core::transform::Transform,
    prelude::*,
    ecs::{Entity},
    assets::{ProgressCounter, Handle, Prefab},
};

use log::*;
use std::collections::HashMap;
use std::mem;


// Load state
#[derive(Debug)]
pub struct DummyLoad{
    progress: ProgressCounter,
    my_assets: HashMap<String, Entity>,
    game_assets: HashMap<String, Entity>,
}

impl SimpleState for DummyLoad {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting {:?}", self);
        let world = data.world;

        self.loady(world);
        self.initialise_card(world);
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Stopping {:?}", self);
    }

    #[allow(clippy::type_complexity)]
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;
        // Checks if we are still loading data
        if self.progress.is_complete() {
            debug!("Debug Assets loaded");
            self.un_loady(world);
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
        let card1_ent = Card::build(card1, world, &mut self.progress);
        self.game_assets.insert("TestCard1".to_string(), card1_ent);

        let mut card2 = Card::new(Suit::Spade, 2);
        card2.set_floating(ARENA_WIDTH - 100., ARENA_HEIGHT / 2.);
        let card2_ent = Card::build(card2, world, &mut self.progress);
        self.game_assets.insert("TestCard2".to_string(), card2_ent);

        let mut clickables = world.write_storage::<Clickable>();
        if let Some(click) = clickables.get_mut(card1_ent) {
            click.set_click(ClickAction::DragTo(vec![card2_ent]));
        }
    }


    fn loady(&mut self, world: &mut World) {
        // Start loading animated loady
        let load_prefab = (*world.read_resource::<Handle<Prefab<LoadScreen>>>()).clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 99.0);

        let ent = world.create_entity()
        .with(load_prefab)
        .with(transform)
        .build();
        LoadScreen::start_anim(world, ent);

        self.my_assets.insert("load_screen".to_string(), ent);
    }

    fn un_loady(&mut self, world: &mut World) {
        for ent in self.my_assets.values() {
            let _ = world.delete_entity(*ent);
        }
    }
}
