use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.0, 0.7, 0.8).into()),
        ..Default::default()
    });
    
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(5.0, 4.0, 2.0),
        ..Default::default()
    });
    
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-3.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
