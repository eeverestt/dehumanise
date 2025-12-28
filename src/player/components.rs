use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct CameraEular {
    pub pitch: f32,
    pub yaw: f32,
}
