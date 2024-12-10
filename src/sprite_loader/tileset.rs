use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::GameState;

pub struct TileLoaderPlugin;

impl Plugin for TileLoaderPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(GameState::Playing), setup)
        //     .insert_resource(LevelSelection::index(0)).

        //     register_ldtk_entity::<TestTiles>("TestTiles");
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("./tilesets/test2.ldtk").into(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Default, Bundle, LdtkEntity)]
pub struct TestTiles {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
