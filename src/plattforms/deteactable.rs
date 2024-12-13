use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_color::palettes::css::{ORANGE, WHITE};

use crate::utils::vec2;

#[derive(Component)]
pub struct Stopper;

pub fn spawn_platform(commands: &mut Commands, pos: Vec3, size: Vec2) {
    commands.spawn((
        Sprite::from_color(WHITE, vec2(10., 100.)),
        Visibility::Hidden,
        RigidBody::Static,
        Collider::rectangle(10., 100.),
        Transform::from_xyz({ pos.x - size.x / 2. }, pos.y, pos.z),
        Stopper,
    ));
    commands.spawn((
        Sprite::from_color(WHITE, vec2(10., 100.)),
        Visibility::Hidden,
        RigidBody::Static,
        Collider::rectangle(10., 100.),
        Transform::from_xyz({ pos.x + size.x / 2. }, pos.y, pos.z),
        Stopper,
    ));
    commands.spawn(super::new_plattform(pos, size));
}
