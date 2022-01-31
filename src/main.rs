mod camera;
mod plane;

use bevy::prelude::*;
use camera::CameraPlugin;
use plane::PlanePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(PlanePlugin)
        .run();
}
