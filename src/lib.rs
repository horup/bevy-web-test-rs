use bevy::{prelude::*};
use wasm_bindgen::prelude::*;
mod systems;
pub use systems::*;
use bevy::window::WindowResized;

pub fn all(app: &mut AppBuilder) {
    app
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_ui.system())
    .add_startup_system(spawn_sprites.system())
    .add_startup_system(spawn_camera.system())
    .add_system(window.system())
    .insert_resource(WindowDescriptor {
        width:1024.,
        height:786.,
        ..Default::default()
    });
}

#[cfg(target_arch = "wasm32")]
pub fn window_size(mut windows:ResMut<Windows>, mut window_resized_events: EventWriter<WindowResized>,) {
    let window = web_sys::window().expect("no global `window` exists");
    let width:f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height:f32 = window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
        if window.width() != width || window.height() != height {
            window.update_actual_size_from_backend(width as u32, height as u32);
            window_resized_events.send(WindowResized {
                id:window.id(),
                height:height,
                width:width
            });
            info!("Resolution set {:?},{:?}", window.width(), window.height());
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn web_only(app: &mut AppBuilder) {
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_system(window_size.system());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native_only(app: &mut AppBuilder) {
}

#[cfg(target_arch = "wasm32")]
pub fn wasm_test(wd:Res<WindowDescriptor>) {
}

#[wasm_bindgen]
pub fn start() {
    let mut app = App::build();
    
    all(&mut app);
    
    #[cfg(target_arch = "wasm32")]
    web_only(&mut app);
    #[cfg(not(target_arch = "wasm32"))]
    native_only(&mut app);

    #[cfg(target_arch = "wasm32")]
    app.add_system(wasm_test.system());

    app.run();
}