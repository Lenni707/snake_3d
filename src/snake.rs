use bevy::prelude::*;

use crate::camera::PlayerCamera;
use crate::grid::Grid;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake);
    }
}

#[derive(Component)]
pub struct Snake {
    body: Vec<Entity>,
    dir: IVec3,
}

#[derive(Component)]
pub struct SnakeSegment {
    pos: IVec3,
}

fn spawn_snake(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<Grid>,
) {
    let head_pos = IVec3::new(0, 0, 0);
    let tail_pos = IVec3::new(-1, 0, 0);

    let head_mesh = meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size * 0.9)));
    let body_mesh = meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size * 0.85)));

    let head_mat = materials.add(Color::srgb_u8(255, 220, 50));
    let body_mat = materials.add(Color::srgb_u8(255, 220, 50));

    let head = cmd
        .spawn((
            // head
            Mesh3d(head_mesh),
            MeshMaterial3d(head_mat),
            Transform::from_translation(grid.cell_to_world(head_pos)),
            SnakeSegment { pos: head_pos },
        ))
        .id();

    let tail = cmd
        .spawn((
            // body
            Mesh3d(body_mesh),
            MeshMaterial3d(body_mat),
            Transform::from_translation(grid.cell_to_world(tail_pos)),
            SnakeSegment { pos: tail_pos },
        ))
        .id();

    cmd.spawn((
        Snake {
            // whole snake (owns the data)
            body: vec![head, tail],
            dir: IVec3::X,
        },
        PlayerCamera,
    ));
}

fn move_snake(mut snake_q: Query<&mut Snake>, mut segment_q: Query<&mut SnakeSegment>) {
    let Ok(snake) = snake_q.single_mut() else {
        return;
    };

    for i in 1..snake.body.len() {
        let next_pos = segment_q.get(snake.body[i + 1]).unwrap().pos;

        segment_q.get_mut(snake.body[i]).unwrap().pos = next_pos;
    }

    let mut head = segment_q.get_mut(snake.body[0]).unwrap();
    head.pos += snake.dir
}

fn sync_transforms(
    // update segment entity positions based on the previous updated ones
    mut q: Query<(&SnakeSegment, &mut Transform), Changed<SnakeSegment>>, // changed ist so eine richtig tuffe bevy method
    grid: Res<Grid>,
) {
    for (seg, mut transform) in &mut q {
        transform.translation = grid.cell_to_world(seg.pos)
    }
}
