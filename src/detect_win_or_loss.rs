use bevy::prelude::*;

use crate::{
    alien::{Alien, ALIEN_HEIGHT},
    despawn_component_type, load_assets,
    player::PLAYER_HEIGHT,
    start_game_on_button_press::start_game_on_button_press,
    GameStates, PADDING, TEXT_COLOR, WORLD_HEIGHT,
};

pub struct DetectWinOrLossPlugin;

#[derive(Component)]
struct OnEndScreen;

impl Plugin for DetectWinOrLossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_win_or_loss.run_if(in_state(GameStates::Game)),
        );
        app.add_systems(OnEnter(GameStates::End), setup_end_screen);
        app.add_systems(
            Update,
            start_game_on_button_press.run_if(in_state(GameStates::End)),
        );
        app.add_systems(
            OnExit(GameStates::End),
            despawn_component_type::<OnEndScreen>,
        );
    }
}

#[derive(Resource)]
struct EndState {
    player_did_win: bool,
}

fn detect_win_or_loss(
    aliens: Query<(Entity, &Transform), With<Alien>>,
    mut game_state: ResMut<NextState<GameStates>>,
    mut cmd: Commands,
) {
    if aliens.is_empty() {
        game_state.set(GameStates::End);
        cmd.insert_resource(EndState {
            player_did_win: true,
        });
    }
    for (_, transform) in &aliens {
        if transform.translation.y
            <= -WORLD_HEIGHT / 2.0 + ALIEN_HEIGHT / 2.0 + PLAYER_HEIGHT + PADDING
        {
            game_state.set(GameStates::End);
            cmd.insert_resource(EndState {
                player_did_win: false,
            });
            break;
        }
    }
}

fn setup_end_screen(mut cmd: Commands, assets: Res<load_assets::Assets>, end_state: Res<EndState>) {
    let text = Text::new(if end_state.player_did_win {
        "Super! Du hast gewonnen!"
    } else {
        "Ach, schade, Du hast verloren!"
    });
    cmd.remove_resource::<EndState>();
    let sound = if end_state.player_did_win {
        assets.win_sound.clone()
    } else {
        assets.game_over_sound.clone()
    };

    cmd.spawn((
        OnEndScreen,
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
    ))
    .with_children(|p| {
        p.spawn((
            text,
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 32.0,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ));
        p.spawn((
            Text::new("Drücke den Schießenknopf für einen weiteren Versuch!"),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 32.0,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ));
        p.spawn(AudioPlayer::new(sound));
    });
}
