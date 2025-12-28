use crate::camera::components::*;
use crate::player::components::*;
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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MeshMaterial3d(materials.add(asset_server.load("test texture 16x.png"))),
        Mesh3d(meshes.add(Capsule3d::new(1., 1.))),
        Transform::from_xyz(0., 0., -4.),
        children![(
            CameraArm,
            children![(Camera3d::default(), CameraEular::default())]
        )],
    ));
}
