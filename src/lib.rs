use bevy::{prelude::*};
use wasm_bindgen::prelude::*;
mod systems;
pub use systems::*;

pub fn all(app: &mut AppBuilder) {
    app
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_ui.system())
    .add_startup_system(spawn_sprites.system())
    .insert_resource(WindowDescriptor {
        width:1024.,
        height:786.,
        ..Default::default()
    });
}

#[cfg(target_arch = "wasm32")]
pub fn web_only(app: &mut AppBuilder) {
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
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