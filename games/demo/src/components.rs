use bevy::prelude::*;

use crate::paddle::PaddleType;

#[derive(Component, Debug)]
pub struct Paddle {
    pub paddle_type: PaddleType,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub vector: Vec3,
}

// impl Default for Velocity {
//     fn default() -> Self {
//         Self {
//             vector: Vec3::default(),
//         }
//     }
// }
