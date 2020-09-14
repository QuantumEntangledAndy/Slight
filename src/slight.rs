use crate::card::{Card, Suit};
use crate::utils::mouse_ray;
use crate::mousetracking::MouseTracking;
use crate::boundingbox::BoundingBox;

use amethyst::{
    core::transform::Transform,
    core::math::{Vector2},
    prelude::*,
    input::{InputEvent, Button},
    winit::MouseButton,
    ecs::Join,
};

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1024.0;

pub struct Slight;

impl SimpleState for Slight {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_card(world);
    }

    fn handle_event(&mut self, data:StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Input(InputEvent::ButtonPressed(Button::Mouse(MouseButton::Left))) = &event {
            // Clicking
            if let Some(m_ray) = mouse_ray(data.world) {
                let m_pos = m_ray.at_distance(0.);
                let m_pos2 = Vector2::new(m_pos[0], m_pos[1]);
                let mut mouse_trackings = data.world.write_storage::<MouseTracking>();
                let transforms = data.world.read_storage::<Transform>();
                let boxes = data.world.read_storage::<BoundingBox>();
                let closest = (&mut mouse_trackings, &transforms, &boxes).join().filter(|(_mt, ct, cb)| cb.hit_ww_trans(&ct, &m_pos2)).max_by(|(_, a, _), (_, b, _)| (a.translation()[2]).partial_cmp(&b.translation()[2]).unwrap_or(std::cmp::Ordering::Equal));
                if let Some(mut closest) = closest {
                    let (mt, _, _) = &mut closest;
                    mt.activate(&m_pos2);
                }
            }

        }

        Trans::None
    }
}

/// Initialises one card on the left, and one on the right.
fn initialise_card(world: &mut World) {
    world.register::<Card>();
    world.register::<BoundingBox>();

    let mut card1 = Card::new(Suit::Heart, 1);
    card1.set_floating(100., ARENA_HEIGHT / 2.);
    Card::build(card1, world);

    let mut card2 = Card::new(Suit::Spade, 2);
    card2.set_floating(ARENA_WIDTH - 100., ARENA_HEIGHT / 2.);
    Card::build(card2, world);
}
