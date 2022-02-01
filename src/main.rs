mod camera;
mod obstacle;
mod plane;
mod world;

use bevy::prelude::*;
use camera::CameraPlugin;
use obstacle::ObstaclePlugin;
use plane::PlanePlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(PlanePlugin)
        .add_plugin(WorldPlugin)
        .run();
}
