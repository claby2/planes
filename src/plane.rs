use std::f32::consts::PI;

use bevy::prelude::*;

pub const MAXIMUM_OFFSET: f32 = 20.0;
pub const ALTITUDE: f32 = 10.0;
const ACCELERATION: f32 = 25.0;

#[derive(Debug)]
pub struct PlanePlugin;

#[derive(Component, Default)]
struct Plane {
    velocity: f32,
    acceleration: f32,
}

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_plane)
            .add_system(control_plane)
            .add_system(move_plane)
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
        .insert(Plane::default());
}

fn control_plane(keyboard_input: Res<Input<KeyCode>>, mut plane: Query<&mut Plane>) {
    let mut plane = plane.single_mut();
    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::A | KeyCode::Left => {
                plane.velocity = 0.0;
                plane.acceleration = -ACCELERATION;
            }
            KeyCode::D | KeyCode::Right => {
                plane.velocity = 0.0;
                plane.acceleration = ACCELERATION;
            }
            _ => {}
        }
    }
}

fn move_plane(time: Res<Time>, mut plane: Query<(&mut Transform, &mut Plane), With<Plane>>) {
    let (mut transform, mut plane) = plane.single_mut();

    // Update velocity if necessary
    if !((transform.translation.x == -MAXIMUM_OFFSET && plane.velocity < 0.0)
        || (transform.translation.x == MAXIMUM_OFFSET && plane.velocity > 0.0))
    {
        plane.velocity += plane.acceleration * time.delta_seconds();
    }

    // Update x translation
    transform.translation.x += plane.velocity;
    transform.translation.x = transform
        .translation
        .x
        .clamp(-MAXIMUM_OFFSET, MAXIMUM_OFFSET);

    // Update y translation relative to x translation
    transform.translation.y = ALTITUDE + 0.01 * transform.translation.x.powi(2);
}

fn rotate_plane(mut transform: Query<&mut Transform, With<Plane>>) {
    let mut transform = transform.single_mut();
    transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        (transform.translation.x * transform.translation.x.abs() * PI)
            / (4.0 * MAXIMUM_OFFSET.powi(2)),
    );
}
