use bevy::prelude::*;

use crate::plane::MAXIMUM_OFFSET;

const TILE_SIZE: f32 = 2.0 * MAXIMUM_OFFSET;
const TILE_SPEED: f32 = 100.0;
const TILE_COLOR: Color = Color::rgb(0.65, 0.8, 0.44);
const NUMBER_OF_TILES: u8 = 10;

#[derive(Debug)]
pub struct WorldPlugin;

#[derive(Debug)]
struct TileHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct Tile;

fn spawn_tile(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    tile_number: u8,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, TILE_SIZE, -(f32::from(tile_number) * TILE_SIZE)),
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

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_mesh = meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE }));
    let tile_material = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        metallic: 0.0,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        ..StandardMaterial::default()
    });

    // Generate initial tiles
    for i in 0..NUMBER_OF_TILES + 1 {
        spawn_tile(&mut commands, tile_mesh.clone(), tile_material.clone(), i);
    }

    // Spawn ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: TILE_SIZE * f32::from(NUMBER_OF_TILES),
        })),
        material: tile_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -(f32::from(NUMBER_OF_TILES) / 2.0) * TILE_SIZE),
        ..PbrBundle::default()
    });

    // Save tile handles as resource for later use
    commands.insert_resource(TileHandles {
        mesh: tile_mesh,
        material: tile_material,
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });
}

fn move_tiles(time: Res<Time>, mut tiles: Query<&mut Transform, With<Tile>>) {
    for mut tile in tiles.iter_mut() {
        tile.translation.z += TILE_SPEED * time.delta_seconds();

        let target_translation = Vec3::new(tile.translation.x, 0.0, tile.translation.z);
        tile.translation = tile.translation.lerp(target_translation, 0.08);
    }
}

fn replace_tile(
    mut commands: Commands,
    tile_handles: Res<TileHandles>,
    tiles: Query<(Entity, &Transform), With<Tile>>,
) {
    for (tile_entity, tile) in tiles.iter() {
        if tile.translation.z >= TILE_SIZE {
            // Despawn the current tile and spawn a new tile
            commands.entity(tile_entity).despawn_recursive();
            spawn_tile(
                &mut commands,
                tile_handles.mesh.clone(),
                tile_handles.material.clone(),
                NUMBER_OF_TILES,
            );
        }
    }
}
