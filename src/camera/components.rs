use bevy::prelude::*;

#[derive(Component)]
pub struct CameraEular {
    pub pitch: f32,
    pub yaw: f32,
}
