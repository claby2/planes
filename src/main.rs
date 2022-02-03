mod camera;
mod collider;
mod menu;
mod obstacle;
mod plane;
mod world;

use bevy::prelude::*;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use menu::MenuPlugin;
use obstacle::ObstaclePlugin;
use plane::PlanePlugin;
use world::WorldPlugin;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    Menu,
    Game,
}

fn despawn_all(entities: Query<Entity>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_all))
        .add_plugin(CameraPlugin)
        .add_plugin(ColliderPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(PlanePlugin)
        .add_plugin(WorldPlugin)
        .run();
}
