use crate::camera::components::*;
use crate::camera::resources::CameraEular;
use crate::player::components::*;
use crate::settings::resources::Keymap;
use bevy::math::I8Vec3;
use bevy::prelude::*;

pub fn apply_player_velocity(
    player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keymap: Res<Keymap>,
    time: Res<Time>,
    camera_eular: Res<CameraEular>,
) {
    for mut player_transform in player_query {
        let yaw = Quat::from_rotation_y(camera_eular.yaw);

        let z = keyboard_input.pressed(keymap.back) as i8
            - keyboard_input.pressed(keymap.forward) as i8;

        let x =
            keyboard_input.pressed(keymap.left) as i8 - keyboard_input.pressed(keymap.right) as i8;

        let dir = I8Vec3::new(x, 0, z);

        player_transform.translation += yaw * dir.as_vec3() * time.delta_secs();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(material.clone()),
        Visibility::default(),
    ));

    commands.spawn((
        Visibility::default(),
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Visibility::default(),
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

    commands.spawn((
        Visibility::default(),
        MeshMaterial3d(materials.add(asset_server.load("test texture 16x.png"))),
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        Transform::from_xyz(0., 0., -4.),
        Player,
        children![(
            CameraArm,
            children![(Camera3d::default())],
            Transform::from_xyz(12., 31., 52.)
        )],
    ));
}
