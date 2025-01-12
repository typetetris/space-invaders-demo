use bevy::prelude::*;

use crate::load_assets;
use crate::start_game_on_button_press::start_game_on_button_press;

use super::GameStates;

use super::TEXT_COLOR;

const SHOOTING_COLOR: Color = Color::srgb(0.666, 0.831, 0.0);
const MOVEMENT_COLOR: Color = Color::srgb(1.0, 0.502, 0.502);

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Splash), setup_splash)
            .add_systems(Update, update_splash.run_if(in_state(GameStates::Splash)))
            .add_systems(
                OnExit(GameStates::Splash),
                super::despawn_component_type::<OnSplashScreen>,
            )
            .add_systems(OnExit(GameStates::Splash), cleanup_splash_timer)
            .add_systems(
                Update,
                start_game_on_button_press.run_if(in_state(GameStates::Splash)),
            );
    }
}

pub(crate) const WAITING_TIME: u32 = 5;

#[derive(Component)]
pub(crate) struct OnSplashScreen;

#[derive(Resource)]
pub(crate) struct SplashTimer(Timer);

#[derive(Component)]
pub(crate) struct UpdateableSplashText;

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
                width: Val::Px(350.0),
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
        parent.spawn((
            UpdateableSplashText,
            Text::new(format!("Start in {} Sekunden!", WAITING_TIME)),
            TextColor(TEXT_COLOR),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 32.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(20.0)),
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
            Text::new("Oder drück den Schießenknopf."),
            TextColor(TEXT_COLOR),
            TextFont {
                font: assets.orbitron_font.clone(),
                font_size: 16.0,
                ..default()
            },
            Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ));
    });
    cmd.insert_resource(SplashTimer(Timer::from_seconds(
        WAITING_TIME as f32,
        TimerMode::Once,
    )));
}

pub(crate) fn update_splash(
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
    splash_text: Single<Entity, (With<UpdateableSplashText>, With<Text>)>,
    mut writer: TextUiWriter,
    mut game_state: ResMut<NextState<GameStates>>,
) {
    if timer.0.tick(time.delta()).finished() {
        game_state.set(GameStates::Game);
    } else {
        let seconds_left = (timer.0.remaining_secs().ceil() as isize).max(0);
        *writer.text(*splash_text, 0) = format!("Start in {} Sekunden!", seconds_left);
    }
}

fn cleanup_splash_timer(mut cmd: Commands) {
    cmd.remove_resource::<SplashTimer>();
}
