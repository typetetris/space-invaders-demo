use bevy::prelude::*;

use crate::{despawn_component_type, GameStates, DISPLAY_HEIGHT, WORLD_HEIGHT};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            OnExit(GameStates::Game),
            despawn_component_type::<OnGameScreen>,
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = WORLD_HEIGHT / DISPLAY_HEIGHT;
    commands.spawn((Camera2d, projection));
}
