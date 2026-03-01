use bevy::prelude::*;
use rand::random_range;

use crate::camera::PlayerCamera;
use crate::grid::Grid;
use crate::controls::Direction;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MoveTimer(Timer::from_seconds(0.4, TimerMode::Repeating)))
            .add_systems(Startup, (spawn_snake, spawn_food))
            .add_systems(Update, (move_snake, sync_transforms, eat_food).chain());
    }
}

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Component)]
pub struct Snake {
    body: Vec<Entity>,
    pub dir: Direction,
    pub last_horizontal_dir: Direction,
    pub score: u32,
}

#[derive(Component)]
pub struct SnakeSegment {
    pos: IVec3,
}

#[derive(Resource)]
pub struct SnakeAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub fn spawn_snake(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<Grid>,
) {
    let head_pos = IVec3::new(1, 0, 2);
    let tail_pos = IVec3::new(0, 0, 2);

    let head_mesh = meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size * 0.9)));
    let body_mesh = meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size * 0.85)));

    let head_mat = materials.add(Color::srgb_u8(255, 165, 0));
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
            Mesh3d(body_mesh.clone()), // handles kann man einfach clonen ist ez
            MeshMaterial3d(body_mat.clone()),
            Transform::from_translation(grid.cell_to_world(tail_pos)),
            SnakeSegment { pos: tail_pos },
        ))
        .id();

    cmd.spawn((
        Snake {
            // whole snake (owns the data)
            body: vec![head, tail],
            dir: Direction::Right,
            last_horizontal_dir: Direction::Right,
            score: 0
        },
        PlayerCamera,
    ));

    cmd.insert_resource(SnakeAssets {
        mesh: body_mesh,
        material: body_mat,
    });

    println!("snake spanwned");
}

fn move_snake(mut snake_q: Query<&mut Snake>, mut segment_q: Query<&mut SnakeSegment>, time: Res<Time>, mut timer: ResMut<MoveTimer>,) {
    if !timer.0.tick(time.delta()).just_finished() { return; }

    let Ok(snake) = snake_q.single_mut() else {
        return;
    };

    let dir = snake.dir.to_ivec3();

    for i in (1..snake.body.len()).rev() {
        let next_pos = segment_q.get(snake.body[i - 1]).unwrap().pos;

        segment_q.get_mut(snake.body[i]).unwrap().pos = next_pos;
    }

    let mut head = segment_q.get_mut(snake.body[0]).unwrap();
    println!("snake moved pos: {}", head.pos);
    head.pos += dir
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

pub fn add_segment(
    snake: &mut Snake,
    segment_q: &Query<&SnakeSegment>,
    cmd: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    grid: &Grid,
) {
    let tail_entity = *snake.body.last().unwrap();
    let tail_pos = segment_q.get(tail_entity).unwrap().pos; // neues egment spawnt einfach in der letzten reihe weil es dann eh geupdated wird

    let new_segment = cmd.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(grid.cell_to_world(tail_pos)),
        SnakeSegment { pos: tail_pos },
    )).id();

    snake.body.push(new_segment);

    println!("segment added");
}

// -- SHIT FOR APPLES --
#[derive(Component)]
pub struct Food;

pub fn spawn_food(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<Grid>,
) {
    let apple_pos = IVec3::new(
        random_range(grid.origin.x as i32..grid.size),
        random_range(grid.origin.y as i32..grid.size),
        random_range(grid.origin.z as i32..grid.size),
    );
    let apple_mesh = meshes.add(Cuboid::from_size(Vec3::splat(grid.cell_size * 0.9)));
    let apple_mat = materials.add(Color::srgb_u8(255, 0, 0));

    cmd.spawn((
        Mesh3d(apple_mesh),
        MeshMaterial3d(apple_mat),
        Transform::from_translation(grid.cell_to_world(apple_pos)),
        Food,
    ));
}

fn eat_food(
    mut snake_q: Query<&mut Snake>,
    segment_q: Query<&SnakeSegment>,
    mut cmd: Commands,
    grid: Res<Grid>,
    assets: Res<SnakeAssets>,
    food_q: Query<(Entity, &Transform), With<Food>>,
) {
    let Ok(mut snake) = snake_q.single_mut() else { return };

    let head_pos = segment_q.get(snake.body[0]).unwrap().pos;
    let head_world = grid.cell_to_world(head_pos);

    for (entity, transform) in food_q.iter() {
        if transform.translation == head_world {
            cmd.entity(entity).despawn();
            add_segment(
                &mut snake,
                &segment_q,
                &mut cmd,
                assets.mesh.clone(),
                assets.material.clone(),
                &grid,
            );
            snake.score += 1;
        }
    }
}