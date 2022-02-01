use crate::plane::ALTITUDE;
use crate::world::{TILE_INTERPOLATION, TILE_SIZE, TILE_SPEED};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct ObstaclePlugin;

#[derive(Component)]
struct Obstacle;

pub fn spawn_obstacle(commands: &mut Commands, obstacle_scene: Handle<Scene>, tile_number: u8) {
    let mut rng = thread_rng();
    commands
        .spawn_bundle((
            Transform::from_xyz(
                rng.gen_range(-TILE_SIZE / 2.0..TILE_SIZE / 2.0),
                TILE_SIZE + ALTITUDE,
                -(f32::from(tile_number) * TILE_SIZE),
            ),
            GlobalTransform::identity(),
        ))
        .with_children(|children| {
            children.spawn_scene(obstacle_scene);
        })
        .insert(Obstacle);
}

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_obstacles).add_system(despawn_obstacles);
    }
}

fn move_obstacles(time: Res<Time>, mut obstacles: Query<&mut Transform, With<Obstacle>>) {
    for mut obstacle in obstacles.iter_mut() {
        obstacle.translation.z += TILE_SPEED * time.delta_seconds();
        let target_translation =
            Vec3::new(obstacle.translation.x, ALTITUDE, obstacle.translation.z);
        obstacle.translation = obstacle
            .translation
            .lerp(target_translation, TILE_INTERPOLATION);
    }
}

// Despawn obstacles that are out of bounds
fn despawn_obstacles(
    mut commands: Commands,
    obstacles: Query<(Entity, &Transform), With<Obstacle>>,
) {
    for (entity, transform) in obstacles.iter() {
        if transform.translation.z >= TILE_SIZE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
