use crate::mousetracking::MouseTracking;

use amethyst::core::math::{Point3, Vector2, Vector3, Vector4};
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{ActiveCamera, Camera};
use amethyst::window::ScreenDimensions;

#[derive(SystemDesc)]
pub struct MouseTrackingSystem;

impl<'s> System<'s> for MouseTrackingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MouseTracking>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (mut transforms, mut objects, cameras, input, time, act_camera, window): Self::SystemData,
    ) {
        let cam_ent = (*act_camera).entity.expect("Activate camera should be set");
        let camera: &Camera = cameras.get(cam_ent).expect("Active camera not found");
        let camera_pos: Transform = transforms
            .get(cam_ent)
            .expect("Camera should have a transform")
            .clone();

        // Gets mouse coordinates
        if let Some((x, y)) = input.mouse_position() {
            let (x, y) = (x, y);
            let mouse_screen =
                camera.screen_to_world_point(Point3::new(x, y, 1.), window.diagonal(), &camera_pos);
            let mouse_screen4 = Vector4::new(mouse_screen[0], mouse_screen[1], 0., 1.);

            for (object, transform) in (&mut objects, &mut transforms).join() {
                if object.is_active() {
                    let parent_mat = transform.global_view_matrix() * transform.matrix();
                    let current_pos = transform.translation();

                    let mouse_local4 = parent_mat * mouse_screen4;

                    let current_pos2 = Vector2::new(current_pos[0], current_pos[1]);
                    let mouse_local2 = Vector2::new(mouse_local4[0], mouse_local4[1]);
                    let new_pos = object.update(&current_pos2, &mouse_local2, time.delta_seconds());

                    if let Some(new_pos) = new_pos {
                        let new_pos3 = Vector3::new(new_pos[0], new_pos[1], current_pos[2]);
                        transform.set_translation(new_pos3);
                    }
                }
            }
        }
    }
}
