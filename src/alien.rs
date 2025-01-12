use bevy::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens);
    }
}

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>) {
    let alien_texture = asset_server.load("alien.png");
    commands.spawn(Sprite {
        image: alien_texture,
        ..Default::default()
    });
}
