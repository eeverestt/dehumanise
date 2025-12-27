use bevy::prelude::*;

pub struct DebugPlugin;

#[derive(Component)]
struct Cube;

#[derive(Resource, Default)]
struct MustKillAllEntities(bool);

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_kill, spawn_camera, spawn_cube))
            .add_systems(Update, update_cube_rot);
    }
}

fn update_cube_rot(
    cube_query: Query<Entity, With<Cube>>,
    kill: Res<MustKillAllEntities>,
    mut command: Commands,
) {
    for cube_entity in cube_query {
        if kill.0 {
            command.entity(cube_entity).despawn();
        }
    }
}

fn init_kill(mut command: Commands) {
    command.insert_resource(MustKillAllEntities(true));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn spawn_cube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MeshMaterial3d(materials.add(asset_server.load("test texture 16x.png"))),
        Mesh3d(meshes.add(Cuboid::from_size(vec3(3., 3., 3.)))),
        Transform::from_xyz(0., 0., -4.),
        Cube,
    ));
}
