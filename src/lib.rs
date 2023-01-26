use bevy::{prelude::*, time::FixedTimestep};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const FIXED_UPDATE_INTERVAL: f64 = 1.0 / 30.0;

#[wasm_bindgen]
pub fn setup_game() {
    console_log!("Setup game...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera_system)
        .add_startup_system(spawn_sprite_system)
        .add_system(debug_fps_system)
        .run();
}

fn debug_fps_system(time: Res<Time>) {
    let fps = 1.0 / time.delta_seconds();
    console_log!("{}", fps);
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_sprite_system(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(64.0, 64.0, 1.0),
            ..default()
        },
        ..default()
    });
}
