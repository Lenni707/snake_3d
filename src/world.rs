use bevy::prelude::*;
use crate::grid::Grid;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, setup_lighting))
        .add_systems(Update, draw_grid_lines);
    }
}

fn spawn_ground(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Mesh3d(meshes.add(Circle::new(15.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
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
        Transform::from_xyz(40.0, 80.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


fn draw_grid_lines(mut gizmos: Gizmos, grid: Res<Grid>) {
    for i in 0..grid.dimensions.z {
        let pos = Vec3::new(grid.origin.x, grid.origin.y, grid.origin.z + i as f32 * grid.cell_size);
        gizmos.grid(
            Isometry3d::new(pos, Quat::from_rotation_x(0.)),
            UVec2::splat(grid.dimensions.x as u32),
            Vec2::splat(grid.cell_size),
            // Light gray
            LinearRgba::gray(0.2),
        );
    }

    for i in 0..grid.dimensions.y {
        let pos = Vec3::new(grid.origin.x, grid.origin.y + i as f32 * grid.cell_size, grid.origin.z);
        gizmos.grid(
            Isometry3d::new(pos, Quat::from_rotation_x(180.)),
            UVec2::splat(grid.dimensions.x as u32),
            Vec2::splat(grid.cell_size),
            // Light gray
            LinearRgba::gray(0.2),
        );
    }
    
}

// ugfagfa