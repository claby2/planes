use std::f32::consts::PI;

use bevy::prelude::*;

pub const MAXIMUM_OFFSET: f32 = 20.0;
pub const ALTITUDE: f32 = 10.0;
const SPEED: f32 = 50.0;
const ROLL_SPEED: f32 = 2.0;

#[derive(Debug)]
pub struct PlanePlugin;

#[derive(Component)]
struct Plane;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_plane)
            .add_system(control_plane)
            .add_system(rotate_plane);
    }
}

fn setup_plane(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, ALTITUDE, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|children| {
            children.spawn_scene(asset_server.load("plane.glb#Scene0"));
        })
        .insert(Plane);
}

fn control_plane(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform: Query<&mut Transform, With<Plane>>,
) {
    let mut transform = transform.single_mut();

    // Update x translation upon input
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        transform.translation.x -= SPEED * time.delta_seconds();
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        transform.translation.x += SPEED * time.delta_seconds();
    }

    transform.translation.x = transform
        .translation
        .x
        .clamp(-MAXIMUM_OFFSET, MAXIMUM_OFFSET);

    // Update y translation relative to x translation
    transform.translation.y = ALTITUDE + 0.01 * transform.translation.x.powi(2);
}

fn rotate_plane(time: Res<Time>, mut transform: Query<&mut Transform, With<Plane>>) {
    let mut transform = transform.single_mut();

    let roll_offset = (PI / 24.0) * (time.time_since_startup().as_secs_f32() * ROLL_SPEED).sin();

    transform.rotation = Quat::from_rotation_z(
        (transform.translation.x * transform.translation.x.abs() * PI)
            / (3.0 * MAXIMUM_OFFSET.powi(2))
            + roll_offset,
    );
}
