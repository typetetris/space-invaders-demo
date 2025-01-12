use bevy::prelude::*;

use crate::game::OnGameScreen;
use crate::load_assets::Assets;
use crate::{GameStates, WORLD_HEIGHT, WORLD_WIDTH};

pub const ALIEN_HEIGHT: f32 = 9f32;
const ALIEN_WIDTH: f32 = 9f32;
const ALIEN_GAP: f32 = 5f32;
const ALIEN_DOWN_TRAVEL_DISTANCE: f32 = ALIEN_HEIGHT + ALIEN_GAP;
const ALIEN_HORIZONTAL_TRAVEL_DISTANCE: f32 = WORLD_WIDTH - 8.0 * ALIEN_WIDTH - 7.0 * ALIEN_GAP;
const ALIEN_SPEED: f32 = 96f32;

const TIME_TRAVELING_HORIZONTALLY: f32 = ALIEN_HORIZONTAL_TRAVEL_DISTANCE / ALIEN_SPEED;

const TIME_TRAVELING_VERTICALLY: f32 = ALIEN_DOWN_TRAVEL_DISTANCE / ALIEN_SPEED;

// To really do different levels with differently sized aliens and
// different reactions, we should probably invent some kind
// of DSL or even use a scripting language like lua/rhai/whatever
// instead of hardcoding paths and alien properties here.
//
// We also would need a hitpoint system of some kind.
//
// A row of aliens will look like that
// 1 2 3 4 5 6 7 8
// XGXGXGXGXGXGXGX
// with X being an alien and G being the space between aliens, the gap.
// so there are 8 aliens in a row and 7 gaps.
//
// The Movement curve will have to travel WORLD_WIDTH-8*alien_width-7*gap_width
// horizontally.

pub struct AlienPlugin;

#[derive(Component)]
pub struct Alien {
    initial_offset: Vec2,
    setup_time: f32,
}

#[derive(Resource)]
struct AlienMovementCurve(AnimatableKeyframeCurve<Vec2>);

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_alien_movement_curve);
        app.add_systems(OnEnter(GameStates::Game), setup_aliens);
        app.add_systems(Update, move_aliens.run_if(in_state(GameStates::Game)));
    }
}

fn setup_alien_movement_curve(mut commands: Commands) {
    let y_pos_iter = (0..)
        .flat_map(|v| {
            [
                -(v as f32) * ALIEN_DOWN_TRAVEL_DISTANCE,
                -(v as f32) * ALIEN_DOWN_TRAVEL_DISTANCE,
            ]
        })
        .take_while(|v| *v > -WORLD_HEIGHT * 1.5f32);
    let x_pos_iter = [
        0f32,
        ALIEN_HORIZONTAL_TRAVEL_DISTANCE,
        ALIEN_HORIZONTAL_TRAVEL_DISTANCE,
        0f32,
    ]
    .into_iter()
    .cycle();
    let t_pos_iter = [TIME_TRAVELING_HORIZONTALLY, TIME_TRAVELING_VERTICALLY]
        .into_iter()
        .cycle()
        .scan(0.0f32, |state, increment| {
            let result = *state;
            *state += increment;
            Some(result)
        });
    let alien_movement_curve = AnimatableKeyframeCurve::new(
        t_pos_iter.zip(x_pos_iter.zip(y_pos_iter).map(|(x, y)| Vec2::new(x, y))),
    )
    .unwrap();
    commands.insert_resource(AlienMovementCurve(alien_movement_curve));
}

fn setup_aliens(mut commands: Commands, assets: Res<Assets>, time: Res<Time>) {
    let alien_texture = &assets.alien;
    for row in 0..3 {
        let row = row as f32;
        for col in 0..8 {
            let col = col as f32;
            let x_off = -WORLD_WIDTH / 2.0 + col * (ALIEN_WIDTH + ALIEN_GAP);
            let y_off = WORLD_HEIGHT / 2.0 - row * (ALIEN_HEIGHT + ALIEN_GAP);
            commands.spawn((
                Alien {
                    initial_offset: Vec2::new(x_off, y_off),
                    setup_time: time.elapsed_secs(),
                },
                OnGameScreen,
                Sprite {
                    image: alien_texture.clone(),
                    ..Default::default()
                },
                Transform::from_xyz(x_off, y_off, 20.0),
            ));
        }
    }
}

fn move_aliens(
    time: Res<Time>,
    movement_curve: Res<AlienMovementCurve>,
    mut aliens: Query<(&Alien, &mut Transform)>,
) {
    for (alien, mut transform) in &mut aliens {
        let t = (time.elapsed_secs() - alien.setup_time).max(0f32);
        let next_position = alien.initial_offset + movement_curve.0.sample_clamped(t);
        transform.translation.x = next_position.x;
        transform.translation.y = next_position.y;
    }
}
