mod domino;
mod environment;
mod pusher;

use crate::environment::*;
use bevy::{color::palettes::css, prelude::*};
use bevy_rapier3d::prelude::*;
use domino::Domino;
use pusher::Pusher;

const DOMINO_SIZE: Vec3 = Vec3::new(1., 2., 0.4);
const DOMINO_HALF_SIZE: Vec3 = Vec3::new(
    DOMINO_SIZE.x * 0.5,
    DOMINO_SIZE.y * 0.5,
    DOMINO_SIZE.z * 0.5,
);
const DOMINO_DISTANCE: f32 = DOMINO_SIZE.z * 4.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(EnvironmentPlugin)
        .add_systems(Startup, (spawn_dominos, spawn_pusher))
        .add_systems(FixedUpdate, move_pusher)
        .run();
}

// #[hot]
// fn update() {
//     let foo = 5;
//     println!("Hello the value of foo is: {:?}", foo);
// }

fn spawn_dominos(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..20 {
        if i != 5 {
            commands.spawn((
                RigidBody::Dynamic,
                Collider::cuboid(DOMINO_HALF_SIZE.x, DOMINO_HALF_SIZE.y, DOMINO_HALF_SIZE.z),
                Mesh3d(meshes.add(Cuboid::new(DOMINO_SIZE.x, DOMINO_SIZE.y, DOMINO_SIZE.z))),
                MeshMaterial3d(materials.add(Color::from(css::GHOST_WHITE))),
                Domino,
                Transform::from_xyz(0., DOMINO_SIZE.y, i as f32 * DOMINO_DISTANCE), // .rotate_axis(Dir3::Y, PI * 0.5),
            ));
        }
    }
}

fn spawn_pusher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.25;
    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(radius),
        Mesh3d(meshes.add(Sphere::new(radius))),
        MeshMaterial3d(materials.add(Color::from(css::RED))),
        Pusher,
        Transform::from_xyz(0., DOMINO_SIZE.y, -4.),
    ));
}

fn move_pusher(mut query: Query<&mut Transform, With<Pusher>>) {
    let Ok(mut transform) = query.single_mut() else {
        println!("Could not query position!");
        return;
    };
    if transform.translation.z <= 0.5 {
        transform.translation.z += 0.05;
    }
}
