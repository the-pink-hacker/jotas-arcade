use bevy::prelude::*;

use crate::components::Ball;

const BALL_SIZE: f32 = 16.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball_system);
    }
}

fn spawn_ball_system(mut commands: Commands) {
    commands.spawn((
        Ball,
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
