use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    // start logger so we can see errors/warnings/debug while project is running
    amethyst::start_logger(Default::default());

    // get display config path
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // basic application setup
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding to open a window and draw to it.
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D is used to render entities with a 'SpriteRender' component
            .with_plugin(RenderFlat2D::default()),
    )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}