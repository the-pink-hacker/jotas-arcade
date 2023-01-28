use bevy::prelude::*;

use crate::components::Paddle;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddles_system)
            .add_system(move_paddles_system);
    }
}

#[derive(Debug)]
pub enum PaddleType {
    Left,
    Right,
}

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    sprite_bundle: SpriteBundle,
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
                    scale: Vec3::new(16.0, 128.0, 1.0),
                    translation: translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup_paddles_system(mut commands: Commands) {
    const PADDLE_SPACING: f32 = 128.0;

    commands.spawn(PaddleBundle::new(
        Paddle {
            paddle_type: PaddleType::Left,
        },
        Vec3::new(-PADDLE_SPACING, 0.0, 0.0),
    ));

    commands.spawn(PaddleBundle::new(
        Paddle {
            paddle_type: PaddleType::Right,
        },
        Vec3::new(PADDLE_SPACING, 0.0, 0.0),
    ));
}

fn move_paddles_system(mut query: Query<(&Paddle, &mut Transform)>, input: Res<Input<KeyCode>>) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0;

        match paddle.paddle_type {
            PaddleType::Left => {
                if input.pressed(KeyCode::W) {
                    direction += 1;
                }
                if input.pressed(KeyCode::S) {
                    direction -= 1;
                }
            }
            PaddleType::Right => {
                if input.pressed(KeyCode::Up) {
                    direction += 1;
                }
                if input.pressed(KeyCode::Down) {
                    direction -= 1;
                }
            }
        };

        transform.translation += Vec3::new(0.0, 1.0 * direction as f32, 0.0);
    }
}
