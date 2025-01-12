use bevy::prelude::*;

use crate::{game::OnGameScreen, load_assets, GameStates, PADDING, WORLD_HEIGHT, WORLD_WIDTH};

pub struct PlayerPlugin;

const PLAYER_MAX_SPEED: f32 = 128f32;
const PLAYER_IMAGE_HEIGHT: f32 = 256f32;
const PLAYER_IMAGE_WIDTH: f32 = 210f32;

pub const PLAYER_HEIGHT: f32 = 16f32;
pub const PLAYER_WIDTH: f32 = PLAYER_IMAGE_WIDTH * PLAYER_HEIGHT / PLAYER_IMAGE_HEIGHT;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Game), setup_player);
        app.add_systems(Update, move_player.run_if(in_state(GameStates::Game)));
    }
}

#[derive(Component)]
struct Player;

#[derive(Event)]
pub struct PlayerShot {
    pub x: f32,
    pub y: f32,
}

fn setup_player(mut cmd: Commands, assets: Res<load_assets::Assets>) {
    let player_sprite = assets.player.clone();
    cmd.spawn((
        Player,
        OnGameScreen,
        Transform::from_xyz(0.0, -WORLD_HEIGHT / 2.0 + PADDING, 20.0),
        Sprite {
            image: player_sprite,
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        },
    ));
}

fn move_player(
    gamepads: Query<(Entity, &Gamepad)>,
    player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut player_shot_event_writer: EventWriter<PlayerShot>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player = &mut *player.into_inner();
    for (_entity, gamepad) in &gamepads {
        if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
            let reading: f32 = if left_stick_x.abs() >= 0.2 {
                if left_stick_x < 0.0 {
                    (left_stick_x + 0.2) / 0.8
                } else {
                    (left_stick_x - 0.2) / 0.8
                }
            } else {
                0.0
            };

            player.translation.x += reading * PLAYER_MAX_SPEED * time.delta().as_secs_f32();
        }
        if gamepad.just_pressed(GamepadButton::South) {
            player_shot_event_writer.send(PlayerShot {
                x: player.translation.x,
                y: -WORLD_HEIGHT / 2.0 + PADDING + PLAYER_HEIGHT / 2.0,
            });
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player.translation.x -= PLAYER_MAX_SPEED * time.delta().as_secs_f32();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player.translation.x += PLAYER_MAX_SPEED * time.delta().as_secs_f32();
    }
    player.translation.x = player.translation.x.max(-WORLD_WIDTH / 2.0);
    player.translation.x = player.translation.x.min(WORLD_WIDTH / 2.0);
    if keyboard_input.just_pressed(KeyCode::Space) {
        player_shot_event_writer.send(PlayerShot {
            x: player.translation.x,
            y: -WORLD_HEIGHT / 2.0 + PADDING + PLAYER_HEIGHT / 2.0,
        });
    }
}
