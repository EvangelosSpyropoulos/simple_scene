use bevy::{
    prelude::*,
    input::mouse::MouseMotion
};

fn main() {
    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotation)
        .add_system_to_stage(CoreStage::PostUpdate, camera_controls)
        .run();
}

#[derive(Component)]
struct Rotator;

#[derive(Component)]
struct Camera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane{size: 30.0})),
        material: materials.add(Color::GRAY.into()),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.0, 0.7, 0.8).into()),
        ..Default::default()
    })
    .insert(Rotator);
    
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, 1.0, -4.0),
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 1.0,
            ring_radius: 0.5,
            subdivisions_segments: 10,
            subdivisions_sides: 5
        })),
        material: materials.add(Color::SALMON.into()),
        ..Default::default()
    });
    
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(5.0, 4.0, 2.0),
        ..Default::default()
    });
    
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(Camera);
}

fn rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotator>>
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(1.0 * time.delta_seconds()));
    }
}

fn camera_controls(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut x_axis_value = 0.0;

    for event in mouse_motion_events.iter() {
        x_axis_value += event.delta.x;
    }

    for mut transform in query.iter_mut() {
            transform.translation = Mat3::from_rotation_y(x_axis_value * time.delta_seconds()) * transform.translation;
            transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}