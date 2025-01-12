use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::prelude::*;

use crate::load_assets;

use super::GameStates;
use super::TEXT_COLOR;

pub struct GamepadWaitPlugin;

impl Plugin for GamepadWaitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::WaitForGamepad), gamepad_wait_setup)
            .add_systems(
                Update,
                gamepad_wait.run_if(in_state(GameStates::WaitForGamepad)),
            )
            .add_systems(
                OnExit(GameStates::WaitForGamepad),
                super::despawn_component_type::<OnGamepadWaitScreen>,
            );
    }
}

#[derive(Component)]
pub(crate) struct OnGamepadWaitScreen;

pub(crate) fn gamepad_wait_setup(mut cmd: Commands, assets: Res<load_assets::Assets>) {
    cmd.spawn((
        OnGamepadWaitScreen,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Warte auf die Verbindung\nzum Gamepad..."),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ));
    });
}

pub(crate) fn gamepad_wait(
    mut gamepad_events: EventReader<GamepadConnectionEvent>,
    mut game_state: ResMut<NextState<GameStates>>,
) {
    for event in gamepad_events.read() {
        match event.connection {
            bevy::input::gamepad::GamepadConnection::Connected { .. } => {
                game_state.set(GameStates::Splash)
            }
            bevy::input::gamepad::GamepadConnection::Disconnected => {}
        }
    }
}
