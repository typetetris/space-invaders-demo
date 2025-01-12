use bevy::prelude::*;

use crate::{player::PlayerShot, WORLD_HEIGHT};

pub struct BulletsPlugin;

const BULLET_SPEED: f32 = 256f32;

#[derive(Component)]
pub struct Bullet;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullet);
        app.add_systems(Update, move_bullets);
    }
}

fn spawn_bullet(
    mut player_shot_event_reader: EventReader<PlayerShot>,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut shot_event = None;
    for event in player_shot_event_reader.read() {
        shot_event = Some(event);
    }
    if let Some(shot_event) = shot_event {
        let bullet_graphics = asset_server.load("bullet.png");
        cmd.spawn((
            Bullet,
            Transform::from_xyz(shot_event.x, shot_event.y, 20.0),
            Sprite {
                image: bullet_graphics,
                ..Default::default()
            },
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
