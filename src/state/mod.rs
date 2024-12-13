use crate::utils::vec3;
use crate::GameState;
use bevy::{math::*, prelude::*};

pub struct StorePlugin;
#[derive(Resource)]
pub struct Store {
    pub collected: u32,
    pub deaths: u32,
    pub health: i32,
}
impl Default for Store {
    fn default() -> Self {
        Self {
            collected: 0,
            deaths: 0,
            health: 5,
        }
    }
}

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Store::default())
            .add_systems(Update, check_health);
    }
}

// fn setup(mut commands: Commands) {
//     commands.spawn(Store::default());
// }

fn check_health(store: Res<Store>, mut state: ResMut<NextState<GameState>>) {
    if store.health <= 0 {
        state.set(GameState::GameOver);
    }
}
