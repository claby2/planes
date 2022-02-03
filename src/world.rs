use crate::{obstacle, plane::MAXIMUM_OFFSET, AppState};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const CLEAR_COLOR: Color = Color::rgb(0.43, 0.80, 0.98);

pub const NUMBER_OF_TILES: u8 = 10;
pub const TILE_SIZE: f32 = 2.0 * MAXIMUM_OFFSET;
pub const TILE_SPEED: f32 = 100.0;
pub const TILE_INTERPOLATION: f32 = 0.08;
const TILE_COLOR: Color = Color::rgb(0.65, 0.8, 0.44);
const COIN_SPAWN_PROBABILITY: f64 = 0.5;

#[derive(Debug)]
pub struct WorldPlugin;

#[derive(Debug)]
struct WorldHandles {
    tile_mesh: Handle<Mesh>,
    tile_material: Handle<StandardMaterial>,
    obstacle_scene: Handle<Scene>,
}

#[derive(Component)]
struct Tile;

fn spawn_tile(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    tile_number: u8,
) {
    // Spawn base plane
    commands
        .spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, TILE_SIZE, -(f32::from(tile_number) * TILE_SIZE)),
            ..PbrBundle::default()
        })
        .insert(Tile);
}

fn spawn_objects(commands: &mut Commands, obstacle_scene: Handle<Scene>, tile_number: u8) {
    let mut rng = thread_rng();
    if rng.gen_bool(COIN_SPAWN_PROBABILITY) {
        obstacle::spawn_obstacle(commands, obstacle_scene, tile_number);
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_world))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(move_tiles)
                    .with_system(replace_tile),
            );
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ClearColor(CLEAR_COLOR));

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
    commands.insert_resource(WorldHandles {
        tile_mesh,
        tile_material,
        obstacle_scene: asset_server.load("obstacle.glb#Scene0"),
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn move_tiles(time: Res<Time>, mut tiles: Query<&mut Transform, With<Tile>>) {
    for mut tile in tiles.iter_mut() {
        tile.translation.z += TILE_SPEED * time.delta_seconds();

        let target_translation = Vec3::new(tile.translation.x, 0.0, tile.translation.z);
        tile.translation = tile
            .translation
            .lerp(target_translation, TILE_INTERPOLATION);
    }
}

fn replace_tile(
    mut commands: Commands,
    world_handles: Res<WorldHandles>,
    tiles: Query<(Entity, &Transform), With<Tile>>,
) {
    for (tile_entity, tile) in tiles.iter() {
        if tile.translation.z >= TILE_SIZE {
            // Despawn the current tile and spawn a new tile
            commands.entity(tile_entity).despawn_recursive();
            spawn_tile(
                &mut commands,
                world_handles.tile_mesh.clone(),
                world_handles.tile_material.clone(),
                NUMBER_OF_TILES,
            );
            spawn_objects(
                &mut commands,
                world_handles.obstacle_scene.clone(),
                NUMBER_OF_TILES,
            );
        }
    }
}
