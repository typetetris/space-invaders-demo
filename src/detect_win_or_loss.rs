use std::time::Duration;

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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameStates = GameStates::End)]
enum EndPhase {
    #[default]
    MinimumDisplayTime,
    Abortable,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(EndPhase = EndPhase::MinimumDisplayTime)]
enum Steps {
    #[default]
    One,
    Two,
    Three,
}

impl Plugin for DetectWinOrLossPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<EndPhase>();
        app.add_sub_state::<Steps>();
        app.add_systems(
            Update,
            detect_win_or_loss.run_if(in_state(GameStates::Game)),
        );
        app.add_systems(OnEnter(GameStates::End), setup_end_screen);
        app.add_systems(
            Update,
            start_game_on_button_press.run_if(in_state(EndPhase::Abortable)),
        );
        app.add_systems(
            Update,
            update_end_screen.run_if(in_state(EndPhase::MinimumDisplayTime)),
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
            <= -WORLD_HEIGHT / 2.0 + ALIEN_HEIGHT / 2.0 + PLAYER_HEIGHT / 2.0 + PADDING
        {
            game_state.set(GameStates::End);
            cmd.insert_resource(EndState {
                player_did_win: false,
            });
            break;
        }
    }
}

#[derive(Resource)]
struct EndScreenMinimumDisplayTimer(Timer);

#[derive(Component)]
struct UpdateableText;

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
            UpdateableText,
            Text::new("."),
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
    cmd.insert_resource(EndScreenMinimumDisplayTimer(Timer::new(
        Duration::from_secs(1),
        TimerMode::Repeating,
    )));
}

#[allow(clippy::too_many_arguments)]
fn update_end_screen(
    time: Res<Time>,
    mut end_screen_minimum_display_timer: ResMut<EndScreenMinimumDisplayTimer>,
    mut next_state: ResMut<NextState<EndPhase>>,
    step: Res<State<Steps>>,
    mut next_step_state: ResMut<NextState<Steps>>,
    updateable_text: Single<Entity, With<UpdateableText>>,
    mut text_writer: TextUiWriter,
    mut cmd: Commands,
) {
    if end_screen_minimum_display_timer
        .0
        .tick(time.delta())
        .just_finished()
    {
        match step.get() {
            Steps::One => {
                next_step_state.set(Steps::Two);
                *text_writer.text(*updateable_text, 0) = "..".to_string();
            }
            Steps::Two => {
                next_step_state.set(Steps::Three);
                *text_writer.text(*updateable_text, 0) = "...".to_string();
            }
            Steps::Three => {
                next_state.set(EndPhase::Abortable);
                *text_writer.text(*updateable_text, 0) =
                    "Drücke den Schießenknopf, um es nochmal zu probieren!".to_string();
                cmd.remove_resource::<EndScreenMinimumDisplayTimer>();
            }
        }
    }
}
