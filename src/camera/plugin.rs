use bevy::prelude::*;

use crate::camera::{resources::CameraEular, systems::rotate_camera_arm};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraEular>();
        app.add_systems(Update, rotate_camera_arm);
    }
}
