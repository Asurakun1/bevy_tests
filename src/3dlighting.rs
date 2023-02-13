use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(animate_light_direction)
        .run();
}

#[derive(Component)]
struct Movable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.0);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    //left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.0);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    //right wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.0);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    //cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Movable,
    ));

    // sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(1.5, 1.0, 1.5),
            ..default()
        },
        Movable,
    ));

    /*
    Lights
    */

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    //red point light

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(1.0, 2.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0,
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.2,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    
    //green spot light

    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-1.0, 2.0, 0.0).looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 1600.0,
            color: Color::GREEN,
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        ..default()
    }).with_children(|builder| {
        builder.spawn(PbrBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
            mesh: meshes.add(Mesh::from(shape::Capsule {
                depth: 0.125,
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                emissive: Color::rgba_linear(0.0, 100.0, 0.0, 0.0),
                ..default()
            }),
            ..default()
        });
    });

    //blue point light

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        point_light: PointLight{
            intensity: 1200.0,
            color: Color::BLUE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    }).with_children(|builder| {
        builder.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                emissive: Color::rgba_linear(0.0, 0.0, 100.0, 0.0),
                ..default()
            }),
            ..default()
        });
    });

    //directional sun light

    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },

            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        ..default()
    });


}

fn animate_light_direction(time: Res<Time>, mut query: Query<&mut Transform, With<DirectionalLight>>){
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {

    for mut transform in &mut query{
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }

        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}