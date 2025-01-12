use bevy::prelude::*;

use crate::{DISPLAY_HEIGHT, WORLD_HEIGHT};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = WORLD_HEIGHT / DISPLAY_HEIGHT;
    commands.spawn((Camera2d, projection));
}
