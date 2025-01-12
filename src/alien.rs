use bevy::prelude::*;

use crate::game::OnGameScreen;
use crate::load_assets::Assets;
use crate::{GameStates, WORLD_HEIGHT, WORLD_WIDTH};

pub const ALIEN_HEIGHT: f32 = 9f32;
const ALIEN_WIDTH: f32 = 9f32;
const ALIEN_GAP: f32 = 5f32;
const ALIEN_DOWN_TRAVEL_DISTANCE: f32 = ALIEN_HEIGHT + ALIEN_GAP;
const ALIEN_SPEED: f32 = 96f32;

pub struct AlienPlugin;

#[derive(Component)]
pub struct Alien;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AlienHorizontalMovementDirection {
    Left,
    Right,
}

impl AlienHorizontalMovementDirection {
    fn other(&self) -> Self {
        match self {
            AlienHorizontalMovementDirection::Left => AlienHorizontalMovementDirection::Right,
            AlienHorizontalMovementDirection::Right => AlienHorizontalMovementDirection::Left,
        }
    }
}

#[derive(Debug, Component)]
enum AlienMovementDirection {
    Horizontal(AlienHorizontalMovementDirection),
    Down {
        distance_left_to_travel: f32,
        next: AlienHorizontalMovementDirection,
    },
}

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Game), setup_aliens);
        app.add_systems(Update, move_aliens.run_if(in_state(GameStates::Game)));
    }
}

fn setup_aliens(mut commands: Commands, assets: Res<Assets>) {
    let alien_texture = &assets.alien;
    for row in 0..3 {
        let row = row as f32;
        for col in 0..8 {
            let col = col as f32;
            commands.spawn((
                Alien,
                OnGameScreen,
                Sprite {
                    image: alien_texture.clone(),
                    ..Default::default()
                },
                Transform::from_xyz(
                    -WORLD_WIDTH / 2.0 + col * (ALIEN_WIDTH + ALIEN_GAP),
                    WORLD_HEIGHT / 2.0 - row * (ALIEN_HEIGHT + ALIEN_GAP),
                    20.0,
                ),
            ));
        }
    }
    commands.spawn((
        AlienMovementDirection::Horizontal(AlienHorizontalMovementDirection::Right),
        OnGameScreen,
    ));
}

fn move_aliens(
    time: Res<Time>,
    alien_movement_direction: Single<&mut AlienMovementDirection>,
    mut aliens: Query<&mut Transform, With<Alien>>,
) {
    let delta = ALIEN_SPEED * time.delta().as_secs_f32();
    let alien_movement_direction = &mut *(alien_movement_direction.into_inner());
    let next_movement_direction = match alien_movement_direction {
        AlienMovementDirection::Horizontal(dir) => {
            let mut switch = false;
            for mut trans in &mut aliens {
                match dir {
                    AlienHorizontalMovementDirection::Left => {
                        trans.translation.x -= delta;
                        if trans.translation.x <= -WORLD_WIDTH / 2.0 {
                            switch = true;
                        }
                    }
                    AlienHorizontalMovementDirection::Right => {
                        trans.translation.x += delta;
                        if trans.translation.x >= WORLD_WIDTH / 2.0 {
                            switch = true;
                        }
                    }
                }
            }
            if !switch {
                AlienMovementDirection::Horizontal(*dir)
            } else {
                AlienMovementDirection::Down {
                    distance_left_to_travel: ALIEN_DOWN_TRAVEL_DISTANCE,
                    next: dir.other(),
                }
            }
        }
        AlienMovementDirection::Down {
            ref distance_left_to_travel,
            ref next,
        } => {
            for mut trans in &mut aliens {
                trans.translation.y -= delta;
            }
            if distance_left_to_travel - delta <= 0.0 {
                AlienMovementDirection::Horizontal(*next)
            } else {
                AlienMovementDirection::Down {
                    distance_left_to_travel: distance_left_to_travel - delta,
                    next: *next,
                }
            }
        }
    };
    *alien_movement_direction = next_movement_direction;
}
