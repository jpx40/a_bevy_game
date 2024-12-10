use crate::actions::Actions;
use crate::camera::camera_fit_inside_current_level;
use crate::loading::TextureAssets;
use crate::movement::*;
use crate::plattforms::{self, Plattform};
use crate::utils::vec2;
use crate::GameState;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, camera_follow.run_if(in_state(GameState::Playing)));

        // .add_systems(Update, camera_fit_inside_current_level.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::from_xyz(0.0, -100.0, 0.0),
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0), Vector::NEG_Y * 1500.0)
            .with_movement(1250.0, 0.92, 400.0, (30.0 as Scalar).to_radians()),
        Player,
    ));
    // commands.spawn((
    //     Sprite::from_color(Color::linear_rgb(130.0, 50.0, 50.0), vec2(50., 50.)),
    //     Transform::from_translation(Vec3::new(0., 0., 1.)),
    //     RigidBody::Dynamic,
    //     Collider::rectangle(30.0, 30.0),
    //     Player,
    // ));
    //
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_secs(),
        actions.player_movement.unwrap().y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
fn camera_follow(
    player_transform: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if player_transform.is_empty() {
        return;
    }
    if !camera.is_empty() {
        let mut t = camera.single_mut();
        t.translation.x = player_transform.single().translation.x;
    }
}
