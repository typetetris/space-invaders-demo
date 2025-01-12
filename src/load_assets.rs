use bevy::prelude::*;

pub struct LoadAssetsPlugin;

#[derive(Resource)]
pub struct Assets {
    pub orbitron_font: Handle<Font>,
    pub alien: Handle<Image>,
    pub alien_damaged: Handle<Image>,
    pub bullet: Handle<Image>,
    pub player: Handle<Image>,

    pub bullet_sound: Handle<AudioSource>,
    pub game_over_sound: Handle<AudioSource>,
    pub win_sound: Handle<AudioSource>,
}

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut cmd: Commands) {
    let alien = asset_server.load("alien.png");
    let alien_damaged = asset_server.load("alien_damaged.png");
    let bullet = asset_server.load("bullet.png");
    let player = asset_server.load("player.png");
    let orbitron_font = asset_server.load("fonts/static/Orbitron-Medium.ttf");

    let bullet_sound = asset_server.load("sounds/PhaserShoot.ogg");
    let game_over_sound = asset_server.load("sounds/GameOverSound.ogg");
    let win_sound = asset_server.load("sounds/WinSound.ogg");

    let assets = Assets {
        alien,
        alien_damaged,
        bullet,
        player,
        bullet_sound,
        game_over_sound,
        win_sound,
        orbitron_font,
    };

    cmd.insert_resource(assets);
}
