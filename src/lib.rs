#![allow(clippy::type_complexity)]
mod actions;
mod audio;
mod binds;
mod constants;
mod loading;
mod menu;
mod movement;
mod plattforms;
mod player;
mod state;
mod ui;
mod utils;
use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::movement::*;
use crate::player::PlayerPlugin;
use avian2d::{math::*, prelude::*};
use bevy::app::App;
mod collectables;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
use bevy_simple_text_input::{
    TextInput, TextInputPlugin, TextInputSubmitEvent, TextInputSystem, TextInputTextColor,
    TextInputTextFont,
};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                ActionsPlugin,
                TextInputPlugin,
                InternalAudioPlugin,
                PhysicsPlugins::default().with_length_unit(20.0),
                PlayerPlugin,
                crate::state::StorePlugin,
                crate::ui::UIPlugin,
                CharacterControllerPlugin,
                crate::collectables::CollectablePlugin,
                crate::plattforms::PlatformsPlugin,
            ))
            .insert_resource(Gravity(Vector::NEG_Y * 1000.0));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
