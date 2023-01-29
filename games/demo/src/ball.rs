use bevy::prelude::*;

use crate::components::{Ball, Velocity};

const BALL_SIZE: f32 = 16.0;
const BALL_SPEED: f32 = 128.0;

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
    mut query: Query<(&Velocity, &mut Transform), With<Ball>>,
    time: Res<Time>,
) {
    if let Ok((velocity, mut transform)) = query.get_single_mut() {
        transform.translation += velocity.vector * time.delta_seconds();
    }
}
