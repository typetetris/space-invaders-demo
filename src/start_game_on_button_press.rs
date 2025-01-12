use bevy::prelude::*;

use crate::GameStates;

pub fn start_game_on_button_press(
    gamepads: Query<&Gamepad>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameStates>>,
) {
    for gamepad in &gamepads {
        if gamepad.pressed(GamepadButton::South) {
            game_state.set(GameStates::Game);
        }
    }
    if keyboard_input.pressed(KeyCode::Space) {
        game_state.set(GameStates::Game);
    }
}
