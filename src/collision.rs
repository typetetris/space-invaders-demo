use bevy::prelude::*;

use crate::alien::Alien;
use crate::bullet::Bullet;
use crate::GameStates;

pub struct CollisionPlugin;

const COLLISION_DIST: f32 = 6f32;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collision.run_if(in_state(GameStates::Game)));
    }
}

fn detect_collision(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut aliens: Query<(Entity, &Transform), With<Alien>>,
    mut cmd: Commands,
) {
    for (bullet, bullet_transform) in &bullets {
        for (alien, alien_transform) in &mut aliens {
            if (alien_transform.translation.x - bullet_transform.translation.x).abs()
                <= COLLISION_DIST
                && (alien_transform.translation.y - bullet_transform.translation.y).abs()
                    <= COLLISION_DIST
            {
                cmd.entity(bullet).despawn();
                cmd.entity(alien).despawn();
            }
        }
    }
}
