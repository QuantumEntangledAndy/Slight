use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

use log::*;
use modulator::sources::ScalarSpring;
use modulator::Modulator;

enum MouseTrackingState {
    Inactive,
    Active,
}

const EPSILON: f32 = 1e-4;

pub struct MouseTracking {
    state: MouseTrackingState,
    pub modulator_x: ScalarSpring,
    pub modulator_y: ScalarSpring,
}

impl MouseTracking {
    pub fn new() -> Self {
        Self {
            state: MouseTrackingState::Inactive,
            modulator_x: ScalarSpring::new(0.1, 0., 0.),
            modulator_y: ScalarSpring::new(0.1, 0., 0.),
        }
    }

    #[allow(dead_code)]
    pub fn activate_xy(&mut self, x: f32, y: f32) {
        self.activate(&Vector2::new(x, y));
    }

    pub fn activate(&mut self, current_pos: &Vector2<f32>) {
        if let MouseTrackingState::Inactive = self.state {
            self.jump_to(current_pos);
            self.state = MouseTrackingState::Active;
        }
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            MouseTrackingState::Active => true,
            MouseTrackingState::Inactive => false,
        }
    }

    pub fn deactivate(&mut self) {
        if let MouseTrackingState::Active = self.state {
            self.jump_to(&Vector2::new(0., 0.));
            self.state = MouseTrackingState::Inactive;
        }
    }

    pub fn update(
        &mut self,
        current_pos: &Vector2<f32>,
        target_pos: &Vector2<f32>,
        delta_t: f32,
    ) -> Option<Vector2<f32>> {
        if let MouseTrackingState::Active = self.state {
            if (self.modulator_x.value() - current_pos[0]).abs() > EPSILON
                || (self.modulator_y.value() - current_pos[1]).abs() > EPSILON
            {
                // Jump has occured
                warn!(
                    "Jump occured from: {},{} to {},{}",
                    self.modulator_x.value(),
                    self.modulator_y.value(),
                    current_pos[0],
                    current_pos[1]
                );
                self.jump_to(current_pos);
            }
            self.modulator_x.spring_to(target_pos[0]);
            self.modulator_y.spring_to(target_pos[1]);
            self.modulator_x.advance((delta_t * 1e6) as u64);
            self.modulator_y.advance((delta_t * 1e6) as u64);
            let result = Vector2::new(self.modulator_x.value(), self.modulator_y.value());

            let delta = (result - target_pos).magnitude();

            Some(result)
        } else {
            None
        }
    }

    fn jump_to(&mut self, destination: &Vector2<f32>) {
        self.modulator_x.jump_to(destination[0]);
        self.modulator_x.spring_to(destination[0]);
        self.modulator_y.jump_to(destination[1]);
        self.modulator_y.spring_to(destination[1]);
    }
}

impl Component for MouseTracking {
    type Storage = DenseVecStorage<Self>;
}
