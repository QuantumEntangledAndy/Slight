mod components;
mod cache;
mod systems;
mod utils;
mod states;

use crate::states::load;
use crate::states::load::{Load, LoadScreen};
use crate::components::droppoint;

use amethyst::{
    assets::{PrefabLoaderSystemDesc},
    animation::{AnimationBundle},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1024.0;

use env_logger::Env;
use log::info;

fn main() -> amethyst::Result<()> {
    // Logger
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    info!("Application root is: {:?}", app_root);
    let display_config_path = assets_dir.join("config").join("display.ron");

    let binding_path = assets_dir.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<LoadScreen>::default(),
            "load_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<droppoint::DropPointFx>::default(),
            "droppoint_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<SpriteScenePrefab>::default(),
            "sprite_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<load::AnimationId, SpriteRender>::new(
            "load_animation_control",
            "load_sampler_interpolation",
        ))?
        .with_bundle(AnimationBundle::<droppoint::AnimationId, SpriteRender>::new(
            "dropoint_animation_control",
            "dropoint_sampler_interpolation",
        ))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                // For rendering UI
                .with_plugin(RenderUi::default()),
        )?
        // With transform systems for position tracking
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["load_animation_control", "load_sampler_interpolation", "dropoint_animation_control", "dropoint_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            systems::MouseTrackingSystem,
            "mouse_tracking",
            &["input_system"],
        );

    let mut game = Application::new(assets_dir, Load::new(), game_data)?;

    game.run();
    Ok(())
}
