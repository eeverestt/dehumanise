use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::{
    camera::{components::CameraArm, resources::CameraEular},
    settings::resources::Sensitivity,
};

pub fn rotate_camera_arm(
    camera_arm_query: Query<&mut Transform, With<CameraArm>>,
    mut camera_eular: ResMut<CameraEular>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    sensitivity: Res<Sensitivity>,
) {
    for mut camera_arm_transform in camera_arm_query {
        let delta = accumulated_mouse_motion.delta;
        if delta != Vec2::ZERO {
            let delta = -delta * sensitivity.0.as_vec2();

            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

            camera_eular.yaw += delta.x;
            camera_eular.pitch = (camera_eular.pitch + delta.y).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            camera_arm_transform.rotation =
                Quat::from_euler(EulerRot::YXZ, camera_eular.yaw, camera_eular.pitch, 0.);
        }
    }
}
