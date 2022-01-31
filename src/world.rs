use bevy::prelude::*;

use crate::plane::MAXIMUM_OFFSET;

const TILE_SIZE: f32 = 2.0 * MAXIMUM_OFFSET;
const TILE_SPEED: f32 = 1.0;
const NUMBER_OF_TILES: u8 = 10;

#[derive(Debug)]
pub struct WorldPlugin;

#[derive(Debug)]
struct TileMesh(Handle<Mesh>);

#[derive(Component)]
struct Tile;

fn spawn_tile(commands: &mut Commands, tile_mesh: Handle<Mesh>, tile_number: u8) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: tile_mesh,
            transform: Transform::from_xyz(0.0, -TILE_SIZE, -(f32::from(tile_number) * TILE_SIZE)),
            ..PbrBundle::default()
        })
        .insert(Tile);
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_world)
            .add_system(move_tiles)
            .add_system(replace_tile);
    }
}

fn setup_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let tile_mesh = meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE }));

    // Generate initial tiles
    for i in 0..NUMBER_OF_TILES + 1 {
        spawn_tile(&mut commands, tile_mesh.clone(), i);
    }

    // Save tile mesh as resource for later use
    commands.insert_resource(TileMesh(tile_mesh));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });
}

fn move_tiles(mut tiles: Query<&mut Transform, With<Tile>>) {
    for mut tile in tiles.iter_mut() {
        tile.translation.z += TILE_SPEED;

        let target_translation = Vec3::new(tile.translation.x, 0.0, tile.translation.z);
        tile.translation = tile.translation.lerp(target_translation, 0.08);
    }
}

fn replace_tile(
    mut commands: Commands,
    tile_mesh: Res<TileMesh>,
    tiles: Query<(Entity, &Transform), With<Tile>>,
) {
    for (tile_entity, tile) in tiles.iter() {
        if tile.translation.z >= TILE_SIZE {
            // Despawn the current tile and spawn a new tile
            commands.entity(tile_entity).despawn_recursive();
            spawn_tile(&mut commands, tile_mesh.0.clone(), NUMBER_OF_TILES);
        }
    }
}
