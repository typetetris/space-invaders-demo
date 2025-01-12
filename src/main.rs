use alien::AlienPlugin;
use bevy::prelude::*;
use game::GamePlugin;

mod alien;
mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Invaders".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512f32, 512f32).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
            AlienPlugin,
        ))
        .run();
}
