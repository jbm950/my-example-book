use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    asset::Assets,
    camera::Camera2d,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::{Vec2, primitives::{Circle, Rectangle, Triangle2d}},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
    window::{Window, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Window".into(),
                resolution: (800, 800).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Circle: left, large, blue
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.5, 1.0))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));

    // Triangle: center, medium, orange
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(-70.0, -60.0),
            Vec2::new(70.0, -60.0),
            Vec2::new(0.0, 70.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.1))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Square: right, small, green
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.4))),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}
