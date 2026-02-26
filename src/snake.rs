use bevy::prelude::*;

use crate::grid::Grid;

pub struct SnakePlugin;

impl Plugin for SnakePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake);

    }
}

#[derive(Component)]
pub struct Snake {
    body: Vec<IVec3>,
}

fn spawn_snake(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<Grid>,
) {
    let pos = IVec3::new(0, 0, 0);

    let world_pos = Grid::cell_to_world(&grid, pos);

    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(world_pos.x, world_pos.y, world_pos.z),
        Snake {
            body: vec![pos]
        }
    ));
}