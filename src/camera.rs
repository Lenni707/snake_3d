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
        Transform::from_xyz(1.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera
    ));
}

fn setup_lighting(mut commands: Commands) { 
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.9, 1.0),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(40.0, 80.0, 40.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
