use amethyst::prelude::*;
use amethyst::core::math::{Point2};
use amethyst::core::{Transform, geometry::Ray};
use amethyst::ecs::{ReadStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{ActiveCamera, Camera};
use amethyst::window::ScreenDimensions;

pub fn mouse_ray(world: &World) -> Option<Ray<f32>> {
    mouse_ray_resource(
        &world.read_storage::<Transform>(),
        &world.read_storage::<Camera>(),
        &world.read_resource::<InputHandler<StringBindings>>(),
        &world.read_resource::<ActiveCamera>(),
        &world.read_resource::<ScreenDimensions>()
    )
}

pub fn mouse_ray_resource<'s>(
        transforms: &ReadStorage<'s, Transform>,
        cameras: &ReadStorage<'s, Camera>,
        input: &InputHandler<StringBindings>,
        act_camera: &ActiveCamera,
        window: &ScreenDimensions) -> Option<Ray<f32>> {

    if let Some(cam_ent) = (*act_camera).entity {
        if let Some(camera) = cameras.get(cam_ent) {
            if let Some(camera_pos) = transforms.get(cam_ent) {
                if let Some((x, y)) = input.mouse_position() {
                    return Some(camera.screen_ray(Point2::new(x, y), window.diagonal(), &camera_pos));
                }
            }
        }
    }
    None
}
