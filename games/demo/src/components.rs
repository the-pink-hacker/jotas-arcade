use bevy::prelude::*;

use crate::paddle::PaddleType;

#[derive(Component, Debug)]
pub struct Paddle {
    pub paddle_type: PaddleType,
}
