use wasm_bindgen::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[wasm_bindgen]
#[no_mangle]
pub fn bevy_start() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (900.0, 600.0).into(),
                title: "Bevy".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    asset_server:Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.1, 0.1))),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::new(100.0, 100.0, 0.0)),
            ..default()
        }
    ));
}