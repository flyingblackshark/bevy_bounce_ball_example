use std::time::Duration;

use bevy::prelude::*;
use bevy::window::*;
use bevy_atmosphere::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_obj::*;
use heron::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(AtmosphereMat::default()) // Default Earth sky
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_plugin(FlyCameraPlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false,
            sky_radius: 10.0,
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
        .init_resource::<SpawnBallState>()
        .add_startup_system(spawn)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(UiCameraBundle::default());
        })
        //.add_system(window_res_system)
        //.add_startup_system(window_fullscreen_system)
        .add_system(spawn_ball)
        .run();
}
fn window_res_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    //window.set_resolution(3240.0, 2160.0);
}
fn window_fullscreen_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_mode(WindowMode::Fullscreen);
}
pub struct SpawnBallState {
    pub timer: Timer,
  
}

impl Default for SpawnBallState {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), true)
        }
    }
}
fn spawn_ball(time: Res<Time>,
    state: Option<ResMut<SpawnBallState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut state = match state {
        Some(s) => s,
        None => return,
    };
    if state.timer.paused() {
        return;
    } else if !state.timer.tick(time.delta()).just_finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    for _i in 0..20 {
        let x: f32 = rng.gen_range(0.0..5.0);
        let y: f32 = rng.gen_range(0.0..200.0);
        let z: f32 = rng.gen_range(0.0..5.0);
        // sphere
        spawn_a_ball(&mut commands,&mut meshes,&mut materials,x,y,z);
    }
}
fn spawn_a_ball(commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    x:f32,y:f32,z:f32){
    commands
    .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 1.0,
            depth: 0.0,
            ..default()
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(PhysicMaterial { restitution:1.0, friction: 0.0, density: 1.0, ..Default::default() })
    .insert(CollisionShape::Sphere { radius: 1.0 });
}
fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.001, 25.0, 250.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(125.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.001, 50.0, 125.0),
            border_radius: None,
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.001, 25.0, 250.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(-125.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.001, 50.0, 125.0),
            border_radius: None,
        });
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(250.0, 25.0, 0.001))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 125.0),
            ..default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(125.0, 50.0, 0.001),
            border_radius: None,
        });
        commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(250.0, 25.0, 0.001))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, -125.0),
            ..default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(125.0, 50.0, 0.001),
            border_radius: None,
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 250.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),

            ..default()
        })
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(125.0, 0.001, 125.0),
            border_radius: None,
        });
    let mut rng = rand::thread_rng();
    for _i in 0..2000 {
        let x: f32 = rng.gen_range(0.0..5.0);
        let y: f32 = rng.gen_range(0.0..200.0);
        let z: f32 = rng.gen_range(0.0..5.0);
        // sphere
        spawn_a_ball(&mut commands,&mut meshes,&mut materials,x,y,z);
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            range: 500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 25.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCamera::default());
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Enemies,
}
