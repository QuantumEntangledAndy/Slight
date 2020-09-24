use crate::utils::mouse_ray;
use crate::components::mousetracking::MouseTracking;
use crate::components::boundingbox::BoundingBox;
use crate::components::droppoint::{DropPointFx, DropPoint, DropAction};

use amethyst::{
    core::transform::Transform,
    core::math::{Vector2},
    prelude::*,
    input::{InputEvent, Button},
    winit::MouseButton,
    ecs::{Entity, Join},
    assets::{Prefab, Handle, PrefabLoader, RonFormat},
};

use log::*;

#[derive(Debug)]
pub struct DummyDrop {
    dragging: Entity,
    drop_points: Vec<Entity>,
    temp_assets: Vec<Entity>,
}

impl DummyDrop {
    pub fn new(dragging: Entity, drop_points: Vec<Entity>) -> Self {
        Self{
            dragging,
            drop_points,
            temp_assets: Vec::new(),
        }
    }
}

impl SimpleState for DummyDrop {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting {:?}", self);
        { // Init mouse tracking
            let mut mouse_trackings = data.world.write_storage::<MouseTracking>();
            let start_pos = data.world.read_storage::<Transform>();
            if let Some(mt) = mouse_trackings.get_mut(self.dragging) {
                if let Some(tr) = start_pos.get(self.dragging) {
                    mt.activate(&Vector2::new(tr.translation()[0], tr.translation()[1]));
                }
            }
        }

        { // Init droppoints
            let droppointfx = data.world.exec(|loader: PrefabLoader<'_, DropPointFx>| {
                loader.load(
                    "droppoint/droppoint.ron",
                    RonFormat,
                    (),
                )
            });

            let entities = data.world.entities();
            let mut transforms = data.world.write_storage::<Transform>();
            let mut droppointfxs = data.world.write_storage::<Handle<Prefab<DropPointFx>>>();
            let mut droppoints = data.world.write_storage::<DropPoint>();
            for droppint_ent in &self.drop_points {
                if let Some(tr) = transforms.get(*droppint_ent) {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(tr.translation()[0], tr.translation()[1], tr.translation()[2]+0.1);
                    let droppoint = DropPoint::new();
                    let drop_obj = entities.build_entity()
                    .with(droppointfx.clone(), &mut droppointfxs)
                    .with(droppoint, &mut droppoints)
                    .with(transform, &mut transforms)
                    .build();
                    self.temp_assets.push(drop_obj);
                    DropPointFx::start_anim_world(data.world, drop_obj);
                }
            }
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Stopping {:?}", self);
        let world = data.world;

        {
            let mut mouse_trackings = world.write_storage::<MouseTracking>();
            if let Some(mt) = mouse_trackings.get_mut(self.dragging) {
                mt.deactivate();
            }
        }

        {
            for ent in &self.temp_assets {
                let _ = world.delete_entity(*ent);
            }
            self.temp_assets = Vec::new();
        }
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
                    if self.drop_points.contains(entity) {
                        let drops = data.world.read_storage::<DropPoint>();
                        if let Some(dr) = drops.get(*entity) {
                            match dr.drop() {
                                DropAction::None => {},
                                DropAction::GiveTo(_) => {},
                                DropAction::GiveDownTo(_) => {},
                            }
                        }
                    }
                }
            }

        }

        Trans::None
    }
}
