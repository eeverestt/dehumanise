use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraEular {
    pub pitch: f32,
    pub yaw: f32,
}
