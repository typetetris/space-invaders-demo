use alien::AlienPlugin;
use bevy::prelude::*;
use bullet::BulletsPlugin;
use collision::CollisionPlugin;
use detect_win_or_loss::DetectWinOrLossPlugin;
use game::GamePlugin;
use gamepad_wait::GamepadWaitPlugin;
use load_assets::LoadAssetsPlugin;
use player::{PlayerPlugin, PlayerShot};
use splash::SplashScreenPlugin;

const PADDING: f32 = 32f32;

const DISPLAY_WIDTH: f32 = 512f32;
const DISPLAY_HEIGHT: f32 = 512f32;

const WORLD_WIDTH: f32 = 256f32;
const WORLD_HEIGHT: f32 = 256f32;

mod alien;
mod bullet;
mod collision;
mod detect_win_or_loss;
mod game;
mod gamepad_wait;
mod load_assets;
mod player;
mod splash;
mod start_game_on_button_press;

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
            GamepadWaitPlugin,
            SplashScreenPlugin,
            GamePlugin,
            AlienPlugin,
            PlayerPlugin,
            BulletsPlugin,
            CollisionPlugin,
            // FrameTimeDiagnosticsPlugin,
            // LogDiagnosticsPlugin::default(),
            DetectWinOrLossPlugin,
        ))
        .run();
}

const TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
fn despawn_component_type<T: Component>(to_despawn: Query<Entity, With<T>>, mut cmd: Commands) {
    for entity in &to_despawn {
        cmd.entity(entity).despawn_recursive();
    }
}
