use bevy::prelude::*;
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let texture_handle = asset_server.load("branding/rand01.png");
    let aspect = 1.0;

    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    /*
    material handle section
    */

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.7),
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    /*
    end of material handles
    */

    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 1.5)
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: red_material_handle,
        transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    });
}
