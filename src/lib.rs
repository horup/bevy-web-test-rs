use bevy::{prelude::*, window::WindowMode};
use wasm_bindgen::prelude::*;
mod systems;
pub use systems::*;

pub fn all(app: &mut AppBuilder) {
    app
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_ui.system())
    .add_startup_system(spawn_sprites.system())
    ;
}

#[cfg(target_arch = "wasm32")]
pub fn web_only(app: &mut AppBuilder) {
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.insert_resource(WindowDescriptor {
        canvas:Some("main".into()),
        ..Default::default()
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native_only(app: &mut AppBuilder) {
    app.insert_resource(WindowDescriptor {
        ..Default::default()
    });
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