use crate::collectables::Collectable;
use crate::constants::*;
use crate::player::Player;
use crate::sprite_loader::PlatformFile;
use crate::state::Store;
use crate::utils::{vec2, vec3};
use crate::GameState;
pub mod deteactable;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy::render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology};
use bevy_color::palettes::css::RED;
const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

#[derive(Component)]
pub struct Plattform;
#[derive(Component, Default, Reflect)]
struct Tiefe;
pub fn new_plattform(pos: Vec3, size: Vec2) -> (Sprite, Transform, RigidBody, Collider, Plattform) {
    (
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, pos.z),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
        Plattform,
    )
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, apply_reset.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let platforms =
        PlatformFile::load("./sprites/platforms.ron").expect("Failed to load platforms");

    for p in platforms.platforms {
        commands.spawn(new_plattform(p.pos, p.size));
    }
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
    //outside

    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(210000.0, 120.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -275.0, 0.0),
        RigidBody::Static,
        Sensor,
        Tiefe,
        Visibility::Hidden,
        Collider::rectangle(210000.0, 120.0),
        CollidingEntities::default(),
    ));

    //Platforms
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(1100.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -175.0, 0.0),
        RigidBody::Static,
        Plattform,
        Collider::rectangle(1100.0, 50.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(210.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(700.0, -175.0, 0.0),
        RigidBody::Static,
        Plattform,
        Collider::rectangle(210.0, 50.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(300.0, 25.0)),
            ..default()
        },
        Transform::from_xyz(175.0, -35.0, 0.0),
        RigidBody::Static,
        Plattform,
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
        Plattform,
        Collider::rectangle(300.0, 25.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(150.0, 80.0)),
            ..default()
        },
        Plattform,
        Transform::from_xyz(475.0, -110.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));
    commands.spawn((
        Plattform,
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(150.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(-475.0, -110.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));
    // Ramps

    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(-125.0, 80.0),
        Vector::NEG_X * 125.0,
        Vector::X * 125.0,
    );

    commands.spawn((
        Mesh2d(meshes.add(ramp_mesh)),
        MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
        Transform::from_xyz(-275.0, -150.0, 0.0),
        RigidBody::Static,
        Plattform,
        ramp_collider,
    ));

    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(20.0, -40.0),
        Vector::new(20.0, 40.0),
        Vector::new(-20.0, -40.0),
    );

    commands.spawn((
        Mesh2d(meshes.add(ramp_mesh)),
        MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
        Transform::from_xyz(380.0, -110.0, 0.0),
        RigidBody::Static,
        ramp_collider,
    ));
}

// // Define the collision layers
// #[derive(PhysicsLayer, Default)]
// enum Layer {
//     #[default]
//     Default,
//     Blue,
//     Red,
// }

// fn setup2(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     // Spawn blue platform that belongs on the blue layer and collides with blue
//     commands.spawn((
//         Sprite {
//             color: Color::srgb(0.2, 0.7, 0.9),
//             custom_size: Some(Vec2::new(500.0, 25.0)),
//             ..default()
//         },
//         Transform::from_xyz(0.0, -50.0, 0.0),
//         RigidBody::Static,
//         Collider::rectangle(500.0, 25.0),
//         CollisionLayers::new([Layer::Blue], [Layer::Blue]),
//     ));

//     // Spawn red platform that belongs on the red layer and collides with red
//     commands.spawn((
//         Sprite {
//             color: Color::srgb(0.9, 0.3, 0.3),
//             custom_size: Some(Vec2::new(500.0, 25.0)),
//             ..default()
//         },
//         Transform::from_xyz(0.0, -200.0, 0.0),
//         RigidBody::Static,
//         Collider::rectangle(500.0, 25.0),
//         CollisionLayers::new([Layer::Red], [Layer::Red]),
//     ));

//     let marble_radius = 7.5;
//     let marble_mesh = meshes.add(Circle::new(marble_radius));

//     // Spawn blue marbles that belong on the blue layer and collide with blue
//     let blue_material = materials.add(Color::srgb(0.2, 0.7, 0.9));
//     for x in -6..6 {
//         for y in 0..4 {
//             commands.spawn((
//                 Mesh2d(marble_mesh.clone()),
//                 MeshMaterial2d(blue_material.clone()),
//                 Transform::from_xyz(
//                     x as f32 * 2.5 * marble_radius,
//                     y as f32 * 2.5 * marble_radius + 200.0,
//                     0.0,
//                 ),
//                 RigidBody::Dynamic,
//                 Collider::circle(marble_radius as Scalar),
//                 CollisionLayers::new([Layer::Blue], [Layer::Blue]),
//             ));
//         }
//     }

//     // Spawn red marbles that belong on the red layer and collide with red
//     let red_material = materials.add(Color::srgb(0.9, 0.3, 0.3));
//     for x in -6..6 {
//         for y in -4..0 {
//             commands.spawn((
//                 Mesh2d(marble_mesh.clone()),
//                 MeshMaterial2d(red_material.clone()),
//                 Transform::from_xyz(
//                     x as f32 * 2.5 * marble_radius,
//                     y as f32 * 2.5 * marble_radius + 200.0,
//                     0.0,
//                 ),
//                 RigidBody::Dynamic,
//                 Collider::circle(marble_radius as Scalar),
//                 CollisionLayers::new([Layer::Red], [Layer::Red]),
//             ));
//         }
//     }
// }

fn apply_reset(
    mut store: ResMut<Store>,
    mut query: Query<(&mut Sprite, &mut Visibility, &mut CollidingEntities), With<Tiefe>>,
    mut pquery: Query<(&mut Transform, Entity), With<crate::player::Player>>,
) {
    for (mut sprite, mut v, mut colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::srgb(0.2, 0.7, 0.9);
            if *v != Visibility::Hidden {
                *v = Visibility::Hidden;
            }
        } else {
            sprite.color = RED.into();
            if !pquery.is_empty() {
                let (mut t, e) = pquery.single_mut();

                if colliding_entities.contains(&e) {
                    if *v != Visibility::Visible {
                        *v = Visibility::Visible;
                    }
                    t.translation = vec3(0.0, 1000.0, 0.0);
                    store.deaths += 1;
                    store.health -= 1;

                    colliding_entities.clear()
                }
            }
        }
    }
}
