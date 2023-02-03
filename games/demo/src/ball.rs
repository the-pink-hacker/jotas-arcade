use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::{Ball, Collidable, Paddle, Velocity},
    Direction, SCREEN_PADDING, WINDOW_WIDTH_HALF,
};

const BALL_SIZE: f32 = 32.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE / 2.0;
const BALL_SPEED: f32 = 512.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball_system)
            .add_system(check_paddle_collisions_system)
            .add_system(check_wall_collision_system);
    }
}

fn spawn_ball_system(mut commands: Commands) {
    commands.spawn((
        Ball,
        Velocity {
            vector: Vec3::new(BALL_SPEED, 0.0, 0.0),
        },
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(BALL_SIZE, BALL_SIZE, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        },
    ));
}

/// Calculates when a ball hits a paddle
fn check_paddle_collisions_system(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddle_query: Query<(&Transform, &Paddle), (With<Collidable>, Without<Ball>)>,
) {
    if let Ok((mut velocity, transform)) = ball_query.get_single_mut() {
        for (paddle_transform, ball) in paddle_query.iter() {
            // Check velocity to prevent multiple bounces per hit
            if match ball.paddle_type {
                Direction::Right => velocity.vector.x > 0.0,
                Direction::Left => velocity.vector.x < 0.0,
            } {
                if collide(
                    transform.translation,
                    Vec2::new(transform.scale.x, transform.scale.y),
                    paddle_transform.translation,
                    Vec2::new(paddle_transform.scale.x, paddle_transform.scale.y),
                )
                .is_some()
                {
                    velocity.vector.x *= -1.0;
                }
            }
        }
    }
}

/// Calculates when a ball hits a wall
fn check_wall_collision_system(mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>) {
    if let Ok((mut velocity, transform)) = ball_query.get_single_mut() {
        // Check velocity to prevent multiple bounces per hit
        if velocity.vector.x > 0.0 {
            // Right
            if transform.translation.x + BALL_SIZE_HALF >= WINDOW_WIDTH_HALF - SCREEN_PADDING {
                velocity.vector.x *= -1.0;
            }
        }
        // Left
        else if transform.translation.x - BALL_SIZE_HALF <= -WINDOW_WIDTH_HALF + SCREEN_PADDING {
            velocity.vector.x *= -1.0;
        }

        // Check velocity to prevent multiple bounces per hit
        if velocity.vector.y > 0.0 {
            // Down
            if transform.translation.y + BALL_SIZE_HALF >= WINDOW_WIDTH_HALF - SCREEN_PADDING {
                velocity.vector.y *= -1.0;
            }
        }
        // Up
        else if transform.translation.y - BALL_SIZE_HALF <= -WINDOW_WIDTH_HALF + SCREEN_PADDING {
            velocity.vector.y *= -1.0;
        }
    }
}
