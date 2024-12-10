use bevy::{prelude::*, state::commands};
use std::{fmt::format, process::exit, time::Duration};

use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(
                Update,
                set_back_to_menu.run_if(in_state(GameState::GameOver)),
            );
    }
}

#[derive(Component)]
struct GameOverTimer {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/DroidSansMono/DroidSansMNerdFont-Regular.otf");

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            width: Val::Px(280.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("GameOver"),
                TextFont {
                    font,
                    font_size: 35.0,
                    ..default()
                },
            ));
        });

    commands.spawn(GameOverTimer {
        timer: Timer::from_seconds(5., TimerMode::Once),
    });
}

fn set_back_to_menu(
    time: Res<Time>,
    mut state: ResMut<NextState<GameState>>,
    mut timer: Query<&mut GameOverTimer>,
) {
    if timer.is_empty() {
        return;
    }
    let mut timer = timer.single_mut();
    if timer.timer.finished() {
        state.set(GameState::Menu);
    } else {
        timer.timer.tick(time.delta());
    }
}
