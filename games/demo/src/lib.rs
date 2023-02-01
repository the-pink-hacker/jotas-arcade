use std::fmt::Debug;

use bevy::prelude::*;
use components::Velocity;
use wasm_bindgen::prelude::*;

use crate::{ball::BallPlugin, paddle::PaddlePlugin};

mod ball;
mod components;
mod paddle;

pub const WINDOW_WIDTH: u16 = 1920;
pub const WINDOW_HEIGHT: u16 = 1440;
pub const ASPECT_RATIO_WIDTH: u8 = 4;
pub const ASPECT_RATIO_HEIGHT: u8 = 3;

#[wasm_bindgen]
pub struct GameInfo {
    aspect_ratio_width: u8,
    aspect_ratio_height: u8,
}

#[wasm_bindgen]
impl GameInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(aspect_ratio_width: u8, aspect_ratio_height: u8) -> Self {
        Self {
            aspect_ratio_width: aspect_ratio_width,
            aspect_ratio_height: aspect_ratio_height,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn aspect_ratio_width(&self) -> u8 {
        self.aspect_ratio_width
    }

    #[wasm_bindgen(setter)]
    pub fn set_aspect_ratio_width(&mut self, aspect_ratio_width: u8) {
        self.aspect_ratio_width = aspect_ratio_width;
    }

    #[wasm_bindgen(getter)]
    pub fn aspect_ratio_height(&self) -> u8 {
        self.aspect_ratio_height
    }

    #[wasm_bindgen(setter)]
    pub fn set_aspect_ratio_height(&mut self, aspect_ratio_height: u8) {
        self.aspect_ratio_height = aspect_ratio_height;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

impl Direction {
    pub fn toggle(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<Direction> for i8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn postGameSetup(game_info: GameInfo);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[allow(dead_code)]
pub fn debug_log<T>(output: T)
where
    T: Debug,
{
    console_log!("{:?}", output);
}

#[wasm_bindgen]
pub fn setup_game() {
    console_log!("Setup game...");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
                ..default()
            },
            ..default()
        }))
        .add_plugin(BallPlugin)
        .add_plugin(PaddlePlugin)
        .insert_resource(ClearColor(Color::rgb_u8(25, 25, 25)))
        .add_startup_system(setup_camera_system)
        .add_startup_system(javascript_event_system)
        .add_system(apply_velocity_system)
        //.add_system(debug_fps_system)
        .run();
}

/// Runs javascript code after the game is setup
fn javascript_event_system() {
    console_log!("Game setup");
    postGameSetup(GameInfo {
        aspect_ratio_width: ASPECT_RATIO_WIDTH,
        aspect_ratio_height: ASPECT_RATIO_HEIGHT,
    });
}

#[allow(dead_code)]
fn debug_fps_system(time: Res<Time>) {
    let fps = 1.0 / time.delta_seconds();
    console_log!("{}", fps);
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn apply_velocity_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.vector * time.delta_seconds();
    }
}
