use bevy::prelude::*;

use crate::GameStates;

pub fn start_game_on_button_press(
    gamepads: Query<&Gamepad>,
    mut game_state: ResMut<NextState<GameStates>>,
) {
    for gamepad in &gamepads {
        if gamepad.pressed(GamepadButton::South) {
            game_state.set(GameStates::Game);
        }
    }
}
