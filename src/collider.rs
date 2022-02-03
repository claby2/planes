use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider(pub f32);

#[derive(Component)]
struct Visible;

#[derive(Debug)]
pub struct CollideEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Debug)]
pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollideEvent>()
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(detect_collisions));
    }
}

fn detect_collisions(
    mut collide_event: EventWriter<CollideEvent>,
    colliders: Query<(Entity, &GlobalTransform, &Collider)>,
) {
    let colliders: Vec<(Entity, &GlobalTransform, &Collider)> = colliders.iter().collect();
    for (i, (entity_a, transform_a, collider_a)) in colliders.iter().enumerate() {
        for (entity_b, transform_b, collider_b) in colliders.iter().skip(i + 1) {
            let distance = transform_a.translation.distance(transform_b.translation);
            if distance < collider_a.0 + collider_b.0 {
                collide_event.send(CollideEvent {
                    entity_a: *entity_a,
                    entity_b: *entity_b,
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct ColliderDebugPlugin;

impl Plugin for ColliderDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_colliders);
    }
}

fn render_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colliders: Query<(Entity, &Collider), Without<Visible>>,
) {
    for (entity, collider) in colliders.iter() {
        commands
            .entity(entity)
            .with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: collider.0,
                        ..shape::Icosphere::default()
                    })),
                    ..PbrBundle::default()
                });
            })
            .insert(Visible);
    }
}
