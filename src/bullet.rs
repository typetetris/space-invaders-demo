use bevy::prelude::*;

use crate::{game::OnGameScreen, load_assets, player::PlayerShot, GameStates, WORLD_HEIGHT};

pub struct BulletsPlugin;

const BULLET_SPEED: f32 = 256f32;

#[derive(Component)]
pub struct Bullet;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullet.run_if(in_state(GameStates::Game)));
        app.add_systems(Update, move_bullets.run_if(in_state(GameStates::Game)));
    }
}

fn spawn_bullet(
    mut player_shot_event_reader: EventReader<PlayerShot>,
    mut cmd: Commands,
    assets: Res<load_assets::Assets>,
) {
    let mut shot_event = None;
    for event in player_shot_event_reader.read() {
        shot_event = Some(event);
    }
    if let Some(shot_event) = shot_event {
        let bullet_sound = assets.bullet_sound.clone();
        let bullet_graphics = assets.bullet.clone();
        cmd.spawn((
            Bullet,
            OnGameScreen,
            Transform::from_xyz(shot_event.x, shot_event.y, 20.0),
            Sprite {
                image: bullet_graphics,
                ..Default::default()
            },
            AudioPlayer::new(bullet_sound),
        ));
    }
}

fn move_bullets(
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.translation.y += BULLET_SPEED * time.delta().as_secs_f32();
        if bullet.translation.y >= WORLD_HEIGHT / 2.0 {
            cmd.entity(entity).despawn();
        }
    }
}
