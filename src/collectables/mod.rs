use crate::utils::vec3;
use crate::GameState;
use crate::{player::Player, state::Store};
use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};
use std::time::Duration;

pub struct CollectablePlugin;

impl Plugin for CollectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, check)
            .add_systems(Update, (falling, check_falling, check).chain());
    }
}
#[derive(Component, PartialEq)]
enum State {
    Grounded,
    Moving,
}

#[derive(Component)]
pub struct Collectable;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arr = [
        (2034., 1100., 0.),
        (200., 1100., 0.),
        (34., 115., 0.),
        (-220., 140., 0.),
        (290., 940., 0.),
    ];

    for (x, y, z) in arr {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(15.))),
            MeshMaterial2d(materials.add(Color::linear_rgb(1., 0., 0.))),
            Collectable,
            Sensor,
            Transform::from_xyz(x, y, z),
            Visibility::Visible,
            RigidBody::Kinematic,
            Collider::circle(25.0),
            State::Moving,
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
    mut store: ResMut<Store>,
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
                store.collected += 1;
                info!("{}", store.collected);
            }
        }
    }
}

fn check_falling(
    mut collider: Query<(&mut Transform, &mut State, Entity), With<Collectable>>,
    mut collisions: ResMut<Collisions>,
) {
    if collider.is_empty() {
        return;
    }
    for (mut t, mut s, e) in collider.iter_mut() {
        collisions.retain(|contacts| {
            if contacts.entity1 == e {
                if *s != State::Grounded {
                    *s = State::Grounded;
                    t.translation += 10.;
                }
                false
            } else if contacts.entity2 == e {
                if *s != State::Grounded {
                    *s = State::Grounded;
                    t.translation += 10.;
                }
                false
            } else {
                true
            }
        });
    }
}

fn falling(mut collider: Query<(&State, &mut Transform), With<Collectable>>) {
    if collider.is_empty() {
        return;
    }
    for (s, mut t) in collider.iter_mut() {
        if State::Moving == *s {
            t.translation.y -= 5.;
        }
    }
}

fn rotate(time: Res<Time>, mut collider: Query<(&State, &mut Transform), With<Collectable>>) {
    if collider.is_empty() {
        return;
    }
    for (s, mut t) in collider.iter_mut() {
        if State::Grounded == *s {
            let delta_time = time.delta_secs_f64().adjust_precision();
            t.rotate_y(0.1);
        }
    }
}
