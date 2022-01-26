use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Player;

#[derive(Component, Clone, Copy, Debug)]
pub struct Enemy;

#[derive(Component, Clone, Copy, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct Stat {
    pub speed: f32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct KillOnBounds;

#[derive(Component, Clone, Copy, Debug)]
pub struct WrapOnBounds;

#[derive(Component, Clone, Copy, Debug)]
pub struct BlockOnBounds;
