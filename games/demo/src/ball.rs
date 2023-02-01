use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::components::{Ball, Paddle, Velocity};

const BALL_SIZE: f32 = 32.0;
const BALL_SPEED: f32 = 512.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball_system)
            .add_system(update_ball_position_system);
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

fn update_ball_position_system(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    if let Ok((mut velocity, transform)) = ball_query.get_single_mut() {
        let mut movement = velocity.vector;
        for paddle_transform in paddle_query.iter() {
            if collide(
                transform.translation,
                Vec2::new(transform.scale.x, transform.scale.y),
                paddle_transform.translation,
                Vec2::new(paddle_transform.scale.x, paddle_transform.scale.y),
            )
            .is_some()
            {
                movement = Vec3::new(-movement.x, movement.y, movement.z);
            }
        }
        // Check X collisions
        // if transform.translation.x.abs() >= PADDLE_SPACING - BALL_SIZE {
        //     // Check Y collisions for each paddle
        //     for paddle_transform in paddle_query.iter() {
        //         // Y higher boundary
        //         if transform.translation.y
        //             <= paddle_transform.translation.y + (PADDLE_HEIGHT / 2.0) + (BALL_SIZE / 2.0)
        //         {
        //             // Y lower boundary
        //             if transform.translation.y
        //                 >= paddle_transform.translation.y
        //                     - (PADDLE_HEIGHT / 2.0)
        //                     - (BALL_SIZE / 2.0)
        //             {
        //                 ball.direction = ball.direction.toggle();
        //             }
        //         }
        //     }
        // }
        velocity.vector = movement;
    }
}
