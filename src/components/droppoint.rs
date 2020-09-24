use amethyst::{
    prelude::*,
    ecs::{
        prelude::Entity,
        ReadStorage,
        WriteStorage,
        Component,
        DenseVecStorage,
    },
    error::Error,
    animation::{
        AnimationSetPrefab,
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
        EndControl,
    },
    assets::{
        PrefabData,
        ProgressCounter,
    },
    renderer::{
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
    },
    derive::PrefabData,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DropAction {
    None,
    GiveTo(Entity),
    GiveDownTo(Entity),
}

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    DropPointFx,
}

#[derive(Debug, Clone)]
pub struct DropPoint {
    drop_action: DropAction,
}

impl DropPoint {
    pub fn new() -> Self {
        Self {
            drop_action: DropAction::None
        }
    }

    pub fn drop(&self) -> DropAction {
        self.drop_action
    }

    pub fn drop_to(&mut self, ent: Entity) {
        self.drop_action = DropAction::GiveTo(ent);
    }
}

impl Default for DropPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for DropPoint {
    type Storage = DenseVecStorage<Self>;
}


// Graphics Part

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
pub struct DropPointFx {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

impl DropPointFx {
    pub fn start_anim_soon(world: &World, ent: Entity) {
        let handle = world.read_resource::<CallbackQueue>().send_handle();
        handle.send(Box::new(move |world| Self::start_anim_world(world, ent))).expect("Failed to add Callback to CallbackQueue.");
    }

    pub fn start_anim_world(world: &World, ent: Entity) {
        let animation_sets = world.read_component::<AnimationSet<AnimationId, SpriteRender>>();
        let control_sets = world.write_component::<AnimationControlSet<AnimationId, SpriteRender>>();
        Self::start_anim_sets(animation_sets, control_sets, ent);
    }


    pub fn start_anim_sets(animation_sets: ReadStorage<AnimationSet<AnimationId, SpriteRender>>, mut control_sets: WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>, ent: Entity) {
        // For each entity that has AnimationSet
        let entity = ent;
        if let Some(animation_set) = animation_sets.get(ent) {
            // Creates a new AnimationControlSet for the entity
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            // Adds the animation to AnimationControlSet and loops infinitely
            control_set.add_animation(
                AnimationId::DropPointFx,
                &animation_set.get(&AnimationId::DropPointFx).unwrap(),
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }
    }
}
