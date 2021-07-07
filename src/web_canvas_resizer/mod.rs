use bevy::prelude::*;
use bevy::window::WindowResized;


fn resizer(mut windows:ResMut<Windows>, mut window_resized_events: EventWriter<WindowResized>) {
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


pub struct WebCanvasResizerPlugin;

impl Plugin for WebCanvasResizerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(target_arch = "wasm32")]
        app.add_system(resizer.system());
    }
}