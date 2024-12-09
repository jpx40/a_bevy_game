use crate::player::Player;
use crate::utils::vec3;
use crate::GameState;
use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

pub struct CollectablePlugin;

impl Plugin for CollectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, check);
    }
}
#[derive(Component)]
pub struct Collectable;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arr = [
        (20., 100., 0.),
        (200., 100., 0.),
        (20., 100., 0.),
        (20., 40., 0.),
    ];

    for (x, y, z) in arr {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
            Collectable,
            Sensor,
            Transform::from_xyz(x, y, z),
            Visibility::Visible,
            RigidBody::Kinematic,
            Collider::circle(25.0),
            CollidingEntities::default(),
        ));
    }
}

// fn spawn_fruit(
//     pos: Vec3,
// ) -> (
//     Mesh2d,
//     MeshMaterial2d,
//     Collectable,
//     Sensor,
//     Transform,
//     Visibility,
//     RigidBody,
//     Collider,
//     CollidingEntities,
// ) {
//         Mesh2d(meshes.add(Circle::new(25.))),

//         MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
//         Collectable,
//         Sensor,
//         Transform::from_xyz(pos.x, pos.y, pos.z),
//         Visibility::Visible,
//         RigidBody::Kinematic,
//         Collider::rectangle(2100.0, 120.0),
//         CollidingEntities::default(),
//     )
// }

fn check(
    player: Query<Entity, With<Player>>,
    mut collider: Query<(&mut Visibility, &CollidingEntities), With<Collectable>>,
) {
    if player.is_empty() {
        return;
    }
    for (mut v, c_e) in collider.iter_mut() {
        if !(Visibility::Hidden == *v) {
            if c_e.contains(&player.single()) {
                *v = Visibility::Hidden;
            }
        }
    }
}
