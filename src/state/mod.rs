use crate::utils::vec3;
use crate::GameState;
use bevy::{math::*, prelude::*};

pub struct StorePlugin;
#[derive(Resource)]
pub struct Store {
    pub collected: u32,
}
impl Default for Store {
    fn default() -> Self {
        Self { collected: 0 }
    }
}

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Store::default());
    }
}

// fn setup(mut commands: Commands) {
//     commands.spawn(Store::default());
// }
