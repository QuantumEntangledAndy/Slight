use amethyst::{
    core::transform::Transform,
    core::math::{Vector2},
    prelude::*,
    input::{InputEvent, Button},
    winit::MouseButton,
    ecs::{Entity, Join},
    assets::{ProgressCounter, Prefab, Handle, PrefabLoader, RonFormat},
};

use log::*;
use std::collections::HashMap;
use std::mem;


#[derive(Debug)]
struct DummyGive {
    give: Entity,
    to: Entity,
}

impl SimpleState for DummyGive {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting {:?}", self);

    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Stopping {:?}", self);
    }
}
