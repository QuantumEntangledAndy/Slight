use amethyst::{
    ecs::{prelude::Entity},
    error::Error,
    animation::{
            AnimationSetPrefab
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
    Splash,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct SplashScreen {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}
