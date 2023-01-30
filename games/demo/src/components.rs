use bevy::prelude::*;

use crate::Direction;

#[derive(Component, Debug)]
pub struct Paddle {
    pub paddle_type: Direction,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Direction,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub vector: Vec3,
}
