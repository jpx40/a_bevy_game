use crate::constants::*;
use crate::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
const COLOR_PLATFORM: Color = Color::rgb(0.29, 0.31, 0.41);

#[derive(Component)]
pub struct Plattform;

pub fn new_plattform(
    x: f32,
    scale: Vec3,
) -> (
    Sprite,
    Transform,
    RigidBody,
    Collider,
    Visibility,
    Plattform,
) {
    (
        Sprite {
            color: COLOR_PLATFORM,
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(x, (scale.y / 2.0) + WINDOW_LEFT_X, 0.0),
            scale,
            ..Default::default()
        },
        RigidBody::Static,
        Collider::rectangle(x, 0.0 + (scale.y / 2.0)),
        Visibility::Visible,
        Plattform,
    )
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(mut commands: Commands) {
    // commands.spawn(new_plattform(500.0, Vec3::new(75.0, 200.0, 1.0)));
    // commands.spawn(new_plattform(100.0, Vec3::new(50.0, 350.0, 1.0)));
    // commands.spawn(new_plattform(350.0, Vec3::new(150.0, 250.0, 1.0)));
    // commands.spawn(new_plattform(250.0, Vec3::new(15000.0, 50.0, 1.0)));

    // commands.spawn((
    //       Sprite {
    //           color: Color::srgb(0.7, 0.7, 0.8),
    //           custom_size: Some(Vec2::new(1100.0, 50.0)),
    //           ..default()
    //       },
    //       Transform::from_xyz(0.0, -175.0, 0.0),
    //       RigidBody::Static,
    //       Collider::rectangle(1100.0, 50.0),
    //   ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(1100.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -175.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(300.0, 25.0)),
            ..default()
        },
        Transform::from_xyz(175.0, -35.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(300.0, 25.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(300.0, 25.0)),
            ..default()
        },
        Transform::from_xyz(-175.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(300.0, 25.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(150.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(475.0, -110.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(150.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(-475.0, -110.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));
}
