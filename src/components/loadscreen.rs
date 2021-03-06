use amethyst::{
    prelude::*,
    ecs::{prelude::Entity},
    error::Error,
    animation::{
        AnimationSetPrefab,
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
        EndControl,
    },
    assets::{
        PrefabData,
        ProgressCounter
    },
    renderer::{
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
    },
    derive::PrefabData,
};

use serde::{Deserialize, Serialize};

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Load,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct LoadScreen {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

impl LoadScreen {
    pub fn start_anim_soon(world: &World, ent: Entity) {
        let handle = world.read_resource::<CallbackQueue>().send_handle();
        handle.send(Box::new(move |world| Self::start_anim(world, ent))).expect("Failed to add Callback to CallbackQueue.");
    }

    pub fn start_anim(world: &mut World, ent: Entity) {
        let animation_sets = world.read_component::<AnimationSet<AnimationId, SpriteRender>>();
        let mut control_sets = world.write_component::<AnimationControlSet<AnimationId, SpriteRender>>();

        // For each entity that has AnimationSet
        let entity = ent;
        if let Some(animation_set) = animation_sets.get(ent) {
            // Creates a new AnimationControlSet for the entity
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            // Adds the `Fly` animation to AnimationControlSet and loops infinitely
            control_set.add_animation(
                AnimationId::Load,
                &animation_set.get(&AnimationId::Load).unwrap(),
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }
    }
}
