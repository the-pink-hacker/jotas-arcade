use bevy::prelude::*;

use crate::components::Paddle;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddles_system);
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
