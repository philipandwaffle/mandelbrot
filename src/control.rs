use bevy::prelude::*;

use crate::MandelbrotMaterial;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_focus_for_mandelbrot_material);
    }
}
fn update_focus_for_mandelbrot_material(
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
    keys: Res<Input<KeyCode>>,
) {
    if materials.len() == 0 {
        return;
    }

    let speed = 0.01;
    let zoom_rate = 0.1;

    let mut delta_x = 0.0;
    if keys.pressed(KeyCode::A) {
        delta_x += speed;
    }
    if keys.pressed(KeyCode::D) {
        delta_x -= speed;
    }

    let mut delta_y = 0.0;
    if keys.pressed(KeyCode::W) {
        delta_y += speed;
    }
    if keys.pressed(KeyCode::S) {
        delta_y -= speed;
    }

    let mut delta_zoom = 0.0;
    if keys.pressed(KeyCode::Up) {
        delta_zoom += zoom_rate;
    }
    if keys.pressed(KeyCode::Down) {
        delta_zoom -= zoom_rate;
    }

    for material in materials.iter_mut() {
        let zoom_scaler = f32::powf(material.1.focus.zoom, 3.0);
        material.1.focus.x += delta_x / (zoom_scaler * 0.03125);
        material.1.focus.y += delta_y / (zoom_scaler * 0.03125);
        material.1.focus.zoom += delta_zoom;
        println!("{:?}", material.1.focus);
    }
}
