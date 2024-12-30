mod camera;
mod ui;

use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef}
};

use ui::{MaterialSettings, UIPlugin};

// This struct defines the data that will be passed to our shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: Vec3,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    // Add UI input handling
    // Start adding more fun stuff
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UIPlugin,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    ui_mat: Res<MaterialSettings>,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: Vec3::from_array(ui_mat.color),
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn(PointLight {
        ..Default::default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}
