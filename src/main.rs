use alien::AlienPlugin;
use bevy::prelude::*;
use bullet::BulletsPlugin;
use collision::CollisionPlugin;
use detect_win::DetectWinPlugin;
use game::GamePlugin;
use load_assets::LoadAssetsPlugin;
use player::{PlayerPlugin, PlayerShot};

const PADDING: f32 = 32f32;

const DISPLAY_WIDTH: f32 = 512f32;
const DISPLAY_HEIGHT: f32 = 512f32;

const WORLD_WIDTH: f32 = 256f32;
const WORLD_HEIGHT: f32 = 256f32;

mod alien;
mod bullet;
mod collision;
mod detect_win;
mod game;
mod load_assets;
mod player;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, States, Default)]
enum GameStates {
    #[default]
    Startup,
    WaitForGamepad,
    Splash,
    Game,
    End,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Invaders".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(
                            DISPLAY_WIDTH + 2.0 * PADDING,
                            DISPLAY_HEIGHT + 2.0 * PADDING,
                        )
                        .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameStates>()
        .add_event::<PlayerShot>()
        .add_plugins((
            LoadAssetsPlugin,
            GamePlugin,
            AlienPlugin,
            PlayerPlugin,
            BulletsPlugin,
            CollisionPlugin,
            // FrameTimeDiagnosticsPlugin,
            // LogDiagnosticsPlugin::default(),
            DetectWinPlugin,
        ))
        .run();
}
