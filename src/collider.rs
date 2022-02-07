use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider(pub f32);

#[derive(Component)]
struct Visible;

#[derive(Debug)]
pub struct CollideEvent {
    pub entity1: Entity,
    pub entity2: Entity,
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
    for [(entity1, transform1, collider1), (entity2, transform2, collider2)] in
        colliders.iter_combinations()
    {
        let distance = transform1.translation.distance(transform2.translation);
        if distance < collider1.0 + collider2.0 {
            collide_event.send(CollideEvent { entity1, entity2 });
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
