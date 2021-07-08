use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_kira_audio::AudioPlugin;
use wasm_bindgen::prelude::*;
mod systems;
pub use systems::*;


#[cfg(target_arch = "wasm32")]
mod web_canvas_resizer;

pub fn all(app: &mut AppBuilder) {
    app
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin)
    .add_startup_system(play_music.system())
    .add_startup_system(spawn_ui.system())
    .add_startup_system(spawn_sprites.system())
    .add_startup_system(spawn_camera.system())
    .insert_resource(WindowDescriptor {
        width:640.,
        height:480.,
        resize_constraints:WindowResizeConstraints {
            max_width:1024.,
            max_height:768.,
            min_width:640.,
            min_height:480.
        },
        ..Default::default()
    });
}

#[cfg(target_arch = "wasm32")]
pub fn web_only(app: &mut AppBuilder) {
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(web_canvas_resizer::WebCanvasResizerPlugin);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native_only(app: &mut AppBuilder) {
}

#[wasm_bindgen]
pub fn start() {
    let mut app = App::build();
    
    all(&mut app);
    
    #[cfg(target_arch = "wasm32")]
    web_only(&mut app);
    #[cfg(not(target_arch = "wasm32"))]
    native_only(&mut app);

    app.run();
}