use crate::camera::components::CameraEular;
use crate::player::components::Player;
use crate::settings::resources::Keymap;
use bevy::prelude::*;

pub fn apply_player_velocity(
    player_query: Query<(&mut Transform, &CameraEular), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keymap: Res<Keymap>,
    time: Res<Time>,
) {
    for (mut player_transform, camera_eular) in player_query {
        let yaw = Quat::from_rotation_y(camera_eular.yaw);
        let mut dir = Vec3::ZERO;

        if keyboard_input.pressed(keymap.forward) {
            dir.z -= 1.
        }
        player_transform.translation += yaw * dir * time.delta_secs();
    }
}
