use bevy::prelude::*;

use crate::Direction;

#[derive(Component, Debug, Default)]
pub struct Paddle {
    pub paddle_type: Direction,
}

#[derive(Component, Default)]
pub struct Ball;

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub vector: Vec3,
}

#[derive(Component, Default)]
pub struct Collidable;

#[derive(Component, Default)]
pub struct Wall;
