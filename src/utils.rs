use amethyst::prelude::*;
use amethyst::core::math::{Point2, Vector3};
use amethyst::core::{Transform, geometry::Ray};
use amethyst::ecs::{ReadStorage, Entity};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{ActiveCamera, Camera};
use amethyst::window::ScreenDimensions;
use amethyst::assets::{Loader, Handle};
use amethyst::animation::*;

use serde::{Deserialize, Serialize};

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

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum TransformAnimations {
    Scale,
    Rotate,
    Translate,
}

pub fn translate_to(world: &World, entity: Entity, from: &Vector3<f32>, to: &Vector3<f32>, time: f32) -> Handle<Animation<Transform>> {
    let animation = {
            let loader = world.read_resource::<Loader>();

            let sampler = loader.load_from_data(
                Sampler {
                    input: vec![0., 1.],
                    output: vec![
                        SamplerPrimitive::Vec3([from[0], from[1], from[2]]),
                        SamplerPrimitive::Vec3([to[0], to[1], to[2]]),
                    ],
                    function: InterpolationFunction::Linear,
                },
                (),
                &world.read_resource(),
            );

            let animation = loader.load_from_data(
                Animation::new_single(0, TransformChannel::Translation, sampler),
                (),
                &world.read_resource(),
            );
            animation
        };

        let mut control_sets = world.write_component::<AnimationControlSet<TransformAnimations, Transform>>();
        let control_set = get_animation_set(&mut control_sets, entity).unwrap();
        // Adds the `Fly` animation to AnimationControlSet and loops infinitely
        control_set.add_animation(
            TransformAnimations::Translate,
            &animation,
            EndControl::Stay,
            1./time,
            AnimationCommand::Start,
        );

        animation
}
