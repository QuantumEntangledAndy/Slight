use super::drop::DummyDrop;

use crate::utils::mouse_ray;
use crate::components::clickable::{Clickable, ClickAction};
use crate::components::boundingbox::BoundingBox;

use amethyst::{
    core::transform::Transform,
    core::math::{Vector2},
    prelude::*,
    input::{InputEvent, Button},
    winit::MouseButton,
    ecs::{Entity, Join},
};

use log::*;
use std::collections::HashMap;

// Play state
#[allow(dead_code)]
#[derive(Debug)]
pub struct Dummy{
    my_assets: HashMap<String, Entity>,
}

impl SimpleState for Dummy {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting {:?}", self);
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Stopping {:?}", self);
    }

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
                    let clickables = data.world.read_storage::<Clickable>();
                    if let Some(cl) = clickables.get(*entity) {
                        match cl.click() {
                            ClickAction::None => {},
                            ClickAction::DragTo(droppoints) => {
                                return Trans::Push(Box::new(DummyDrop::new(
                                    *entity,
                                    droppoints.to_vec(),
                                )))
                            },
                            ClickAction::DrawFrom(_) => {

                            }
                        }
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
