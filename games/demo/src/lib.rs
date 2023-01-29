use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::paddle::PaddlePlugin;

mod components;
mod paddle;

pub const WINDOW_WIDTH: i16 = 1280;
pub const WINDOW_HEIGHT: i16 = 720;
pub const FIXED_UPDATE_INTERVAL: f32 = 60.0;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn postGameSetup();
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn setup_game() {
    console_log!("Setup game...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_startup_system(setup_camera_system)
        .add_startup_system(javascript_event_system)
        .insert_resource(ClearColor(Color::rgb_u8(25, 25, 25)))
        //.add_system(debug_fps_system)
        .run();
}

/// Runs javascript code after the game is setup
fn javascript_event_system() {
    console_log!("Game setup");
    postGameSetup();
}

#[allow(dead_code)]
fn debug_fps_system(time: Res<Time>) {
    let fps = 1.0 / time.delta_seconds();
    console_log!("{}", fps);
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
