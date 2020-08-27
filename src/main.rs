mod slight;
mod card;
use crate::slight::Slight;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
};
use log::info;

fn main() -> amethyst::Result<()> {
    // Logger
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    info!("Application root is: {:?}", app_root);
    let display_config_path = assets_dir.join("config").join("display.ron");

    let binding_path = assets_dir.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
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
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(assets_dir, Slight, game_data)?;
    game.run();
    Ok(())
}
