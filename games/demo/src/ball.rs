use bevy::prelude::*;

use crate::{
    components::{Ball, Paddle, Velocity},
    paddle::{PADDLE_HEIGHT, PADDLE_SPACING},
    Direction,
};

const BALL_SIZE: f32 = 32.0;
const BALL_SPEED: f32 = 256.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball_system)
            .add_system(update_ball_position_system);
    }
}

fn spawn_ball_system(mut commands: Commands) {
    commands.spawn((
        Ball::default(),
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

fn update_ball_position_system(
    mut ball_query: Query<(&mut Ball, &Velocity, &mut Transform), With<Ball>>,
    paddle_query: Query<&Transform, Without<Ball>>,
    time: Res<Time>,
) {
    if let Ok((mut ball, velocity, mut transform)) = ball_query.get_single_mut() {
        // Check X collisions
        if transform.translation.x.abs() >= PADDLE_SPACING - BALL_SIZE {
            // Check Y collisions for each paddle
            for paddle_transform in paddle_query.iter() {
                // Y higher boundary
                if transform.translation.y
                    <= paddle_transform.translation.y + (PADDLE_HEIGHT / 2.0) + (BALL_SIZE / 2.0)
                {
                    // Y lower boundary
                    if transform.translation.y
                        >= paddle_transform.translation.y
                            - (PADDLE_HEIGHT / 2.0)
                            - (BALL_SIZE / 2.0)
                    {
                        ball.direction = ball.direction.toggle();
                    }
                }
            }
        }

        let direction: i8 = ball.direction.into();
        transform.translation += velocity.vector * direction as f32 * time.delta_seconds();
    }
}
