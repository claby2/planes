use crate::plane::ALTITUDE;
use bevy::prelude::*;

const OFFSET: f32 = 10.0;

#[derive(Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, ALTITUDE + OFFSET, 40.0),
        ..PerspectiveCameraBundle::default()
    });
}
