use bevy::prelude::*;

use crate::{alien::Alien, load_assets};

pub struct DetectWinPlugin;

#[derive(Component)]
struct WinSound;

impl Plugin for DetectWinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_win);
    }
}

fn detect_win(
    aliens: Query<Entity, With<Alien>>,
    already_playing: Query<Entity, With<WinSound>>,
    mut cmd: Commands,
    assets: Res<load_assets::Assets>,
) {
    if aliens.is_empty() && already_playing.is_empty() {
        let winsound = assets.win_sound.clone();
        cmd.spawn((WinSound, AudioPlayer::new(winsound)));
    }
}
