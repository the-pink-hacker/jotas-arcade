use bevy::prelude::*;
use wasm_bindgen::prelude::*;

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

#[derive(Component)]
struct Player;

#[wasm_bindgen]
pub fn setup_game() {
    console_log!("Setup game...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera_system)
        .add_startup_system(spawn_player_system)
        .add_startup_system(javascript_event_system)
        //.add_system(debug_fps_system)
        .add_system(move_player_system)
        .run();
}

fn javascript_event_system() {
    console_log!("Game setup");
    postGameSetup();
}

fn debug_fps_system(time: Res<Time>) {
    let fps = 1.0 / time.delta_seconds();
    console_log!("{}", fps);
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player_system(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(64.0, 64.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player_system(mut query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.translation += Vec3::new(1.0, 0.0, 0.0);
    }
}
