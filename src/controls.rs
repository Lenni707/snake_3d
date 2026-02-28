use bevy::prelude::*;

use crate::snake::{Snake, SnakeAssets, SnakeSegment, add_segment};
use crate::grid::Grid;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, steer_system)
        .add_systems(Update, handle_input);

    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Back,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Left    => Direction::Right,
            Direction::Right   => Direction::Left,
            Direction::Up      => Direction::Down,
            Direction::Down    => Direction::Up,
            Direction::Forward => Direction::Back,
            Direction::Back    => Direction::Forward,
        }
    }

    pub fn to_ivec3(self) -> IVec3 {
        match self {
            Direction::Left    => IVec3::NEG_X,
            Direction::Right   => IVec3::X,
            Direction::Up      => IVec3::Y,
            Direction::Down    => IVec3::NEG_Y,
            Direction::Forward => IVec3::NEG_Z,
            Direction::Back    => IVec3::Z,
        }
    }

   pub fn turn_left(self, last_h: Direction) -> Direction {
    let reference = match self {
        Direction::Up | Direction::Down => last_h,
        _ => self,
    };
    match reference {
        Direction::Forward => Direction::Left,
        Direction::Left    => Direction::Back,
        Direction::Back    => Direction::Right,
        Direction::Right   => Direction::Forward,
        Direction::Up | Direction::Down => Direction::Left, // fallback, shouldn't hit
    }
}

pub fn turn_right(self, last_h: Direction) -> Direction {
    let reference = match self {
        Direction::Up | Direction::Down => last_h,
        _ => self,
    };
    match reference {
        Direction::Forward => Direction::Right,
        Direction::Right   => Direction::Back,
        Direction::Back    => Direction::Left,
        Direction::Left    => Direction::Forward,
        Direction::Up | Direction::Down => Direction::Right, // fallback, shouldn't hit
    }
}
}

fn steer_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut snake_q: Query<&mut Snake>,
) {
    let Ok(mut snake) = snake_q.single_mut() else {
        return;
    };

    let new_dir = if keyboard.just_pressed(KeyCode::ArrowUp) {
        Some(Direction::Up)
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        Some(Direction::Down)
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
        Some(snake.dir.turn_left(snake.last_horizontal_dir))
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        Some(snake.dir.turn_right(snake.last_horizontal_dir))
    } else {
        None
    };

    if let Some(dir) = new_dir {
        if dir != snake.dir.opposite() {
            // remember last horizontal dir before going vertical
            match dir {
                Direction::Up | Direction::Down => {}
                _ => snake.last_horizontal_dir = dir,
            }
            snake.dir = dir;
        }
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut snake_q: Query<&mut Snake>,
    segment_q: Query<&SnakeSegment>,
    mut cmd: Commands,
    grid: Res<Grid>,
    assets: Res<SnakeAssets>
) {
    if !keyboard.just_pressed(KeyCode::KeyX) {
        return;
    }

    let Ok(mut snake) = snake_q.single_mut() else {
        return;
    };

    add_segment(
        &mut snake,
        &segment_q,
        &mut cmd,
        assets.mesh.clone(),
        assets.material.clone(),
        &grid,
    );
}