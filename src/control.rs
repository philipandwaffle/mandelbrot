use bevy::prelude::*;

use crate::globals::Focus;
use crate::globals::GlobalsBuffer;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input);
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut globals_buffer: ResMut<GlobalsBuffer>) {
    let speed = 0.01;
    let zoom_rate = 1.0;
    let mut focus = globals_buffer.focus_buffer.get().clone();
    if keys.pressed(KeyCode::A) {
        focus.x -= speed;
    }
    if keys.pressed(KeyCode::D) {
        focus.x += speed;
    }

    if keys.pressed(KeyCode::W) {
        focus.y += speed;
    }
    if keys.pressed(KeyCode::S) {
        focus.y -= speed;
    }

    if keys.pressed(KeyCode::Up) {
        focus.zoom += zoom_rate;
    }
    if keys.pressed(KeyCode::Down) {
        focus.zoom -= zoom_rate;
    }

    globals_buffer.focus_buffer.set(focus);
    globals_buffer.focus_buffer.set(Focus {
        x: 0.0,
        y: 0.0,
        zoom: 4.0,
    });
}
