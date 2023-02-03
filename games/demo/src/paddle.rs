use bevy::prelude::*;

use crate::{
    components::{Collidable, Paddle},
    Direction, SCREEN_PADDING, WINDOW_HEIGHT_HALF, WINDOW_WIDTH_HALF,
};

const PADDLE_SPEED: f32 = 512.0;

pub const PADDLE_WIDTH: f32 = 32.0;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.0;
pub const PADDLE_HEIGHT: f32 = 256.0;
pub const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.0;

pub const PADDLE_MARGIN: f32 = 32.0;
pub const PADDLE_SPACING: f32 =
    WINDOW_WIDTH_HALF - PADDLE_WIDTH_HALF - SCREEN_PADDING - PADDLE_MARGIN;
const PADDLE_MAX_HEIGHT: f32 = WINDOW_HEIGHT_HALF - PADDLE_HEIGHT_HALF - SCREEN_PADDING;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddles_system)
            .add_system(move_paddles_system);
    }
}

#[derive(Bundle, Default)]
struct PaddleBundle {
    paddle: Paddle,
    sprite_bundle: SpriteBundle,
    collidable: Collidable,
}

impl PaddleBundle {
    fn new(paddle: Paddle, translation: Vec3) -> Self {
        Self {
            paddle: paddle,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                    translation: translation,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

fn setup_paddles_system(mut commands: Commands) {
    commands.spawn(PaddleBundle::new(
        Paddle {
            paddle_type: Direction::Left,
        },
        Vec3::new(-PADDLE_SPACING, 0.0, 0.0),
    ));

    commands.spawn(PaddleBundle::new(
        Paddle {
            paddle_type: Direction::Right,
        },
        Vec3::new(PADDLE_SPACING, 0.0, 0.0),
    ));
}

fn move_paddles_system(
    mut query: Query<(&Paddle, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0;

        match paddle.paddle_type {
            Direction::Left => {
                if input.pressed(KeyCode::W) {
                    direction += 1;
                }
                if input.pressed(KeyCode::S) {
                    direction -= 1;
                }
            }
            Direction::Right => {
                if input.pressed(KeyCode::Up) {
                    direction += 1;
                }
                if input.pressed(KeyCode::Down) {
                    direction -= 1;
                }
            }
        };

        transform.translation.y = (transform.translation.y
            + (PADDLE_SPEED * direction as f32 * time.delta_seconds()))
        .clamp(-PADDLE_MAX_HEIGHT, PADDLE_MAX_HEIGHT);
    }
}
