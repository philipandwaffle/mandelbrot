use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        globals::{GlobalsUniform, GLOBALS_TYPE_HANDLE},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        RenderApp,
    },
};
pub struct GlobalsPlugin;

impl Plugin for GlobalsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, GLOBALS_TYPE_HANDLE, "globals.wgsl", Shader::from_wgsl);
        app.register_type::<GlobalsUniform>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_system(prepare_globals_buffer);
        }
    }
}

#[derive(Default, Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource)]
pub struct Focus {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[derive(Resource, Default)]
pub struct GlobalsBuffer {
    pub focus_buffer: UniformBuffer<Focus>,
}

fn prepare_globals_buffer(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut globals_buffer: ResMut<GlobalsBuffer>,
) {
    let buffer = globals_buffer.focus_buffer.get_mut();
    buffer.x = 0.0;
    buffer.y = 0.0;
    buffer.zoom = 1.0;

    globals_buffer
        .focus_buffer
        .write_buffer(&render_device, &render_queue);
}
