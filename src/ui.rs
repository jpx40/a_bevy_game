use std::{fmt::format, process::exit};

use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

use crate::state::Store;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::Playing), setup)
            .add_systems(Update, update_text)
            .add_systems(Update, update_death_text);
    }
}
#[derive(Component)]
pub struct Count(i32);
#[derive(Component)]
pub struct Counter;
#[derive(Component)]
pub struct DeathCounter;
#[derive(Component)]
pub struct HealthCounter;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/DroidSansMono/DroidSansMNerdFont-Regular.otf");
    // commands.spawn((
    //     // Create a Text with multiple child spans.
    //     Text::new("Jumps: 0"),
    //     TextFont {
    //         font,
    //         font_size: 42.0,
    //         ..default()
    //     },
    //     Counter,
    //     Count(0),
    // ));
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
                Text::new("Collected: 0"),
                TextFont {
                    font: font.clone(),
                    font_size: 35.0,
                    ..default()
                },
                Counter,
            ));
            parent.spawn((
                // Create a Text with multiple child spans.
                Text::new("Deaths: 0"),
                TextFont {
                    font: font.clone(),
                    font_size: 35.0,
                    ..default()
                },
                DeathCounter,
            ));
            parent.spawn((
                // Create a Text with multiple child spans.
                Text::new("Heath: 0"),
                TextFont {
                    font,
                    font_size: 35.0,
                    ..default()
                },
                HealthCounter,
            ));
        });

    // Create a Text with multiple child spans.
}

#[derive(Event)]
pub enum UiAction {
    Increase,
    Clear,
    Nothing,
}

// pub fn update_counter(
//     mut event: EventReader<UiAction>,
//     mut query: Query<&mut Count, With<Counter>>,
// ) {
//     if query.is_empty() {
//         return;
//     }
//     for e in event.read() {
//         match e {
//             UiAction::Increase => {
//                 info!("12121313");
//                 let mut count = query.single_mut();
//                 count.0 = 1 + count.0;
//             }

//             UiAction::Clear => {
//                 let mut count = query.single_mut();
//                 count.0 = 0;
//             }

//             UiAction::Nothing => (),
//         }
//     }
// }

pub fn update_text(store: Res<Store>, mut query: Query<(&mut Text), With<Counter>>) {
    for (mut text) in query.iter_mut() {
        **text = format!("Collected: {}", store.collected);

        // count.0 = 0;
        // **text = format!("{}", count.0);
    }
}

pub fn update_death_text(store: Res<Store>, mut query: Query<(&mut Text), With<HealthCounter>>) {
    for (mut text) in query.iter_mut() {
        **text = format!("Health: {}", store.health);

        // count.0 = 0;
        // **text = format!("{}", count.0);
    }
}

pub fn update_health_text(store: Res<Store>, mut query: Query<(&mut Text), With<DeathCounter>>) {
    for (mut text) in query.iter_mut() {
        **text = format!("Deaths: {}", store.deaths);

        // count.0 = 0;
        // **text = format!("{}", count.0);
    }
}
