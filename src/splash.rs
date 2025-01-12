use std::time::Duration;

use bevy::prelude::*;

use crate::load_assets;
use crate::start_game_on_button_press::start_game_on_button_press;

use super::GameStates;

use super::TEXT_COLOR;

const SHOOTING_COLOR: Color = Color::srgb(0.666, 0.831, 0.0);
const MOVEMENT_COLOR: Color = Color::srgb(1.0, 0.502, 0.502);
const TRANSPARENT: Color = Color::srgba(0.8, 0.8, 0.8, 0.0);
const GAMEPAD_CONNET_TEXT: &str = "Schließe ein Gamepad an!";
const GAMEPAD_FOUND_TEXT: &str = "Gamepad gefunden!";

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Splash), setup_splash)
            .add_systems(
                OnExit(GameStates::Splash),
                super::despawn_component_type::<OnSplashScreen>,
            )
            .add_systems(
                Update,
                start_game_on_button_press.run_if(in_state(GameStates::Splash)),
            )
            .add_systems(Update, update_splash.run_if(in_state(GameStates::Splash)));
    }
}

#[derive(Component)]
pub(crate) struct OnSplashScreen;

#[derive(Component)]
struct GamepadWaitText {
    timer: Timer,
}

pub(crate) fn setup_splash(mut cmd: Commands, assets: Res<load_assets::Assets>) {
    cmd.spawn((
        OnSplashScreen,
        Node {
            row_gap: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            ImageNode::new(assets.gamepad.clone()),
            Node {
                width: Val::Px(175.0),
                ..default()
            },
        ));
        parent.spawn((
            ImageNode::new(assets.keyboard.clone()),
            Node {
                width: Val::Px(350.0),
                ..default()
            },
        ));
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(64.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Bewegen"),
                    TextColor(MOVEMENT_COLOR),
                    TextFont {
                        font: assets.orbitron_font.clone(),
                        font_size: 32.0,
                        ..default()
                    },
                ));
                parent.spawn((
                    Text::new("Schießen"),
                    TextColor(SHOOTING_COLOR),
                    TextFont {
                        font: assets.orbitron_font.clone(),
                        font_size: 32.0,
                        ..default()
                    },
                ));
            });
        parent.spawn((
            GamepadWaitText {
                timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            },
            Text::new(GAMEPAD_CONNET_TEXT),
            TextColor(TRANSPARENT),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 24.0,
                ..default()
            },
        ));
        parent.spawn((
            Text::new("Weiter mit dem Knopf zum Schießen!"),
            TextColor(TEXT_COLOR),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 24.0,
                ..default()
            },
        ));
    });
}

fn update_splash(
    gamepads: Query<&Gamepad>,
    mut gamepad_waittext: Single<(&mut GamepadWaitText, &mut Text, &mut TextColor)>,
    time: Res<Time>,
) {
    let gamepad_present = !gamepads.is_empty();
    let elapsed = gamepad_waittext.0.timer.tick(time.delta()).elapsed_secs();

    let alpha = if gamepad_present {
        1.0
    } else {
        (2.0 - 2.0 * (elapsed - 1.0) * (elapsed - 1.0)).min(1.0)
    };
    let text_ref = &mut *gamepad_waittext.1;
    if gamepad_present && text_ref.as_str() != GAMEPAD_FOUND_TEXT {
        *text_ref = Text::new(GAMEPAD_FOUND_TEXT);
    } else if !gamepad_present && text_ref.as_str() != GAMEPAD_CONNET_TEXT {
        *text_ref = Text::new(GAMEPAD_CONNET_TEXT);
    }
    gamepad_waittext.2.set_alpha(alpha);
}
