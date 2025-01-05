use std::random::random;

use avian2d::{math::*, prelude::*};
use bevy::{ecs::system::RemovedSystem, prelude::*};
use bevy_color::palettes::css::{BLACK, ORANGE};
use glam::vec3;
use ops::asin;
use rand::Rng;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Component)]
pub struct Angle(pub f32);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Magazin(pub i32, pub i32, pub bool);
use crate::{player::Player, utils::AddSystem, GameState, Grounded};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullet.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct JumpEffectParticle;

#[derive(Component)]
pub struct EffectTimer {
    pub timer: Timer,
}

enum EffectColor {
    Red,
    Orange,
}

fn rand_color() -> Color {
    let i = rand::thread_rng().gen_range(1..10);

    if i >= 2 {
        return Color::srgb(1., 0., 0.);
    } else {
        return ORANGE.into();
    }
}
#[derive(Component)]
pub struct Transition {
    pub start: Vec3,
    pub end: Vec3,
    pub modifier: f32,
    pub modifier_end: f32,
}

impl Transition {
    pub fn new(v: &Vec3) -> Self {
        Self {
            start: Vec3 {
                x: v.x,
                y: v.y,
                z: v.z,
            },
            end: rand_end(v),
            modifier: 0.,
            modifier_end: rand::thread_rng().gen_range(1..10) as f32,
        }
    }
}

fn pos_or_neg() -> f32 {
    let i = rand::thread_rng().gen_range(0..3);

    if i == 1 {
        1.
    } else if i == 2 {
        0.0
    } else {
        -1.0
    }
}

fn rand_end(v: &Vec3) -> Vec3 {
    Vec3 {
        x: v.x + { pos_or_neg() * (rand::thread_rng().gen_range(1..5) as f32) },
        y: (v.y + (rand::thread_rng().gen_range(1..10) as f32)),
        z: v.z,
    }
}

fn new_bullet(
    v: &Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    angle: &Angle,
    direction: &Direction,
) -> (
    JumpEffectParticle,
    Transition,
    Transform,
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
) {
    let end = match *direction {
        Direction::Left => vec3(v.x - 10, v.y, v.z),
        Direction::Right => vec3(v.x + 10, v.y, v.z),
        Direction::Up => vec3(v.x, v.y + 10, v.z),
        Direction::Down => vec3(v.x, v.y - 10, v.z),
    };

    (
        JumpEffectParticle,
        Transition {
            start: *v,
            end,
            modifier: 1.,
            modifier_end: 100.,
        },
        Bullet,
        Transform::from_xyz(v.x, v.y, v.z),
        Mesh2d(meshes.add(Circle::new(2.))),
        MeshMaterial2d(materials.add(BLACK)),
    )
}

pub fn move_bullet(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Transition, Entity), With<Bullet>>,
) {
    for (mut transform, mut transition, mut e) in query.iter_mut() {
        if transition.modifier >= transition.modifier_end {
            commands.entity(e).try_despawn();
        } else {
            transform.translation.x = transition.start.x
                + (transition.modifier * (transition.end.x - transition.start.x));
            transform.translation.y = transition.start.y
                + (transition.modifier * (transition.end.y - transition.start.y));
            transition.modifier += 0.5;
        }
    }
}

fn setup(
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_batch(vec![(
        Mesh2d(meshes.add(Circle::new(15.))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1., 0., 0.))),
    )]);
}

pub fn shoot(mut query: Query<&mut Magazin, With<Player>>, keycode: Res<ButtonInput<KeyCode>>) {
    if query.is_empty() {
        return;
    }
    if keycode.pressed(KeyCode::Enter) {
        let mut mag = query.single_mut();

        mag.1 += 1;
    }
}
pub fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Magazin, &Transform, &Angle, &Direction), With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut m, t, a, d) in query.iter_mut() {
        let mut v = Vec::new();
        let mut counter = 0;
        if !m.2 {
            for i in 0..m.1 as usize {
                if m.0 <= 0 {
                    m.0 = 0;
                    break;
                } else {
                    m.0 -= 1;
                    v.push(new_bullet(
                        &t.translation,
                        &mut meshes,
                        &mut materials,
                        a,
                        d,
                    ));
                }
                m.1 = 0;
            }
            commands.spawn_batch(v);
        }
    }
}

// pub fn change_color_effect(mut query: Query<&mut SprTrite, (With<Player>, Added<Grounded>)>) {
//     for t in query.iter() {
//         let mut v = Vec::new();
//         for i in 0..100 {
//             v.push(new_particle(&t.translation, &mut meshes, &mut materials));
//         }
//         commands.spawn_batch(v);
//     }
// }
