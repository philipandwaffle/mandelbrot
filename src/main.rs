// use num_complex::Complex64;
// fn main() {
//     println!("Hello, world!");

//     let m = mandelbrot::Mandelbrot {
//         id: 0,
//         x_min: -2.0,
//         x_max: 1.0,
//         y_max: 1.0,
//         y_min: -1.0,
//         pixel_per_unit: 1000,
//         iteration_max: 10000,
//         centre_of_barrier: Complex64::new(0.0, 0.0),
//     };
//     m.gen_image().save("test_003.jpg");
// }

mod control;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    window::{PresentMode, WindowMode},
};
use control::ControlPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mandelbrot Explore".into(),
                resolution: (1900., 1280.).into(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(MaterialPlugin::<MandelbrotMaterial>::default())
        .add_plugin(ControlPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(8.0, 4.5),
            flip: false,
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(MandelbrotMaterial {
            focus: Focus {
                x: 0.0,
                y: 0.0,
                z_x: 0.0,
                z_y: 0.0,
                zoom: 4.0,
            },
        }),
        ..default()
    });

    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube::default())),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: materials.add(CustomMaterial {}),
    //     ..default()
    // });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
#[derive(Debug, Clone, ShaderType)]
struct Focus {
    x: f32,
    y: f32,
    z_x: f32,
    z_y: f32,
    zoom: f32,
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct MandelbrotMaterial {
    #[uniform(0)]
    focus: Focus,
}
impl Material for MandelbrotMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot_shader.wgsl".into()
    }
}
