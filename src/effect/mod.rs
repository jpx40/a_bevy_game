use std::random::random;

use avian2d::{math::*, prelude::*};
use bevy::{ecs::system::RemovedSystem, prelude::*};
use bevy_color::palettes::css::ORANGE;
use ops::asin;
use rand::Rng;

use crate::{player::Player, utils::AddSystem, GameState, Grounded};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_effect.run_if(in_state(GameState::Playing)));
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
                y: v.y + 5.,
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

fn new_particle(
    v: &Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> (
    JumpEffectParticle,
    Transition,
    Transform,
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
) {
    (
        JumpEffectParticle,
        Transition::new(v),
        Transform::from_xyz(v.x, v.y, v.z),
        Mesh2d(meshes.add(Circle::new(2.))),
        MeshMaterial2d(materials.add(rand_color())),
    )
}

pub fn move_particle(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Transition, Entity), With<JumpEffectParticle>>,
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

pub fn apply_effect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, Added<Grounded>>,
) {
    for t in query.iter() {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(new_particle(&t.translation, &mut meshes, &mut materials));
        }
        commands.spawn_batch(v);
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
