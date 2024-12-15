use std::default;

use crate::{plattforms::deteactable::Stopper, GameState};
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_color::palettes::css::{ORANGE, WHITE};
#[derive(Component)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}
impl Default for Move {
    fn default() -> Self {
        Move::Left
    }
}
pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}
#[derive(Component)]
pub struct Mob;

fn setup(mut commands: Commands) {
    commands.spawn((
        RigidBody::Kinematic,
        Collider::capsule(12.5, 20.0),
        Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
              MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
              Transform::from_xyz(0.0, -100.0, 0.0),
        Mob, Move::default()));
}

pub fn detect_collision(
    mut collision: Query<&mut CollidingEntities, With<Stopper>>,
    mut query: Query<(&mut Move, Entity), With<Mob>>,
) {
    for (mut m, e) in query.iter_mut() {
        for mut c in collision.iter_mut() {
            c.retain(|e2| {
                if *e == *e2 {
                    if *m == Move::Right {
                        *m = Move::Left;
                    } else if *m == Move::Left {
                        *m = Move::Right;
                    }
                    false
                } else {
                    true
                }
            });
        }
    }
}
pub fn move_mob(mut query: Query<(&mut Transform, &Move), With<Mob>>) {
    query.iter_mut().for_each(|(mut t, m)| {
        if *m == Move::Right {
            *t.translation.x += 1;
        } else if *m == Move::Left {
            *t.translation.x -= 1;
        }
    });
}
