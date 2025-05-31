mod domino;
mod environment;

use std::f32::consts::PI;

use crate::environment::*;
use bevy::{color::palettes::css, prelude::*};
use bevy_rapier3d::prelude::*;
// use bevy_simple_subsecond_system::{SimpleSubsecondPlugin, hot};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(EnvironmentPlugin)
        .add_systems(Startup, spawn_dominos)
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
    let domino_size = vec3(1., 2., 0.4);
    let domino_half_size = domino_size * 0.5;
    let domino_distance = domino_size.z * 5.;
    for i in 0..10 {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(domino_half_size.x, domino_half_size.y, domino_half_size.z),
            Mesh3d(meshes.add(Cuboid::new(domino_size.x, domino_size.y, domino_size.z))),
            MeshMaterial3d(materials.add(Color::from(css::BROWN))),
            Transform::from_xyz(0., domino_size.y * 2., i as f32 * domino_distance), // .rotate_axis(Dir3::Y, PI * 0.5),
        ));
    }
}
