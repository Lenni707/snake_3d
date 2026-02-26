use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 2.0, 3.0),
        PlayerCamera
    ));
}

fn setup_lighting(mut cmd: Commands) {
    cmd.spawn
}

