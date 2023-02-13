use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct Orbit;

const X_EXTENT: f32 = 14.;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 7.0, 11.0)
            .looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50.0 }.into()),
        material: materials.add(Color::GRAY.into()),
        ..default()
    });

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 100.0,
                shadows_enabled: true,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::Torus {
                            radius: 1.5,
                            ring_radius: 1.5 / 2.5,
                            ..default()
                        }
                        .into(),
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(10.0, 10.0, 10.0),
                        emissive: Color::WHITE,
                        metallic: 1.0,
                        reflectance: 1.0,
                        ..default()
                    }),
                    ..default()
                },
                Shape,
            ));
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 1.0,
                    ..default()
                }
                .into(),
            ),
            transform: Transform::from_xyz(-5.0, 4.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.4, 0.9),
                metallic: 0.0,
                reflectance: 1.0,
                perceptual_roughness: 0.0,
                ..default()
            }),
            ..default()
        },
        Shape,
    ));

    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 1.0,
                    ..default()
                }
                .into(),
            ),
            transform: Transform::from_xyz(5.0, 5.0, -3.0),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.6, 0.5),
                metallic: 0.0,
                reflectance: 1.0,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
        Shape,
    ));


    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 1.0,
                    ..default()
                }
                .into(),
            ),
            transform: Transform::from_xyz(-2.0, 4.0, -3.0),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.3, 0.6),
                metallic: 0.0,
                reflectance: 1.0,
                perceptual_roughness: 0.0,
                ..default()
            }),
            ..default()
        },
        Shape,
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.0);
    }
}
