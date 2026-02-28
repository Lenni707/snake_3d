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
    grid: Res<Grid>
) {
    let grid_dims = grid.dimensions.as_vec3();
    cmd.spawn((
        Mesh3d(meshes.add(Circle::new(grid.size as f32 * 1.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(144, 255, 100))),
        Transform::from_isometry(Isometry3d::new(Vec3::new(grid_dims.x * 0.25, grid.origin.y, grid_dims.z * 0.25), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)))
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
    let pos = Vec3::new(grid.origin.x, grid.origin.y, grid.origin.z) + grid.dimensions.as_vec3() * 0.25;
    // gizmos.grid_3d(
    //     Isometry3d::new(pos, Quat::IDENTITY),
    //     UVec3::splat(grid.size as u32),
    //     Vec3::splat(grid.cell_size),
    //     LinearRgba::gray(0.3),
    // );

    let cube_size = grid.size as f32 * grid.cell_size;
    gizmos.primitive_3d(
        &Cuboid::new(cube_size, cube_size, cube_size),
        Isometry3d::new(pos, Quat::IDENTITY),
        LinearRgba::RED,
    );
}

// ugfagfa

