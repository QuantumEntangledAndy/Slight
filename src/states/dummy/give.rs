use crate::utils::{translate_to, TransformAnimations};

use amethyst::{
    core::{
        math::{
            Vector3,
            Vector4,
        },
        transform::Transform,
    },
    prelude::*,
    ecs::{Entity},
    assets::Handle,
    animation::{Animation, AnimationControlSet, get_animation_set},
};

use log::*;


pub struct DummyGive {
    give: Entity,
    to: Entity,
    anim: Option<Handle<Animation<Transform>>>
}

impl DummyGive {
    pub fn new(give: Entity, to: Entity) -> Self {
        Self{
            give,
            to,
            anim: None,
        }
    }
}

impl SimpleState for DummyGive {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Starting DummyGive");
        let transforms = data.world.read_storage::<Transform>();
        if let (Some(transform), Some(dest_transform)) = (transforms.get(self.give), transforms.get(self.to)) {
            let init_position = transform.translation();

            let parent_mat = transform.global_view_matrix() * transform.matrix();
            let global_dest_position4 = dest_transform.global_matrix() * Vector4::new(0., 0., 0., 0.);
            let local_dest_position4 = parent_mat * global_dest_position4;
            let local_dest_position3 = Vector3::new(local_dest_position4[0], local_dest_position4[1], local_dest_position4[2]);

            let anim = translate_to(data.world, self.give, init_position, &local_dest_position3, 1.0);
            self.anim = Some(anim);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.anim.is_some() {
            let mut control_sets = data.world.write_component::<AnimationControlSet<TransformAnimations, Transform>>();
            let control_set = get_animation_set(&mut control_sets, self.give).unwrap();
            if ! control_set.has_animation(TransformAnimations::Translate) {
                return Trans::Pop;
            }
        } else {
            return Trans::Pop; // Failed to start
        }

        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Stopping DummyGive");
    }
}
