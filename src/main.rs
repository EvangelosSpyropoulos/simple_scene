use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotation.system())
        .run();
}

struct Rotator;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.0, 0.7, 0.8).into()),
        ..Default::default()
    })
    .insert(Rotator);
    
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(5.0, 4.0, 2.0),
        ..Default::default()
    });
    
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotator>>
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(1.0 * time.delta_seconds()));
    }
}