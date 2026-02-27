use bevy::prelude::*;

use crate::snake::Snake;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, steer_system);
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction{
    Left,
    Right,
    Up,
    Down
}

fn steer_system( // ok dicke todo muss ich alles noch machen
    keyboard: Res<ButtonInput<KeyCode>>,
    real_time: Res<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    snake_q: Query<&Snake>
) {
    let Ok(snake) = snake_q.single() else {
        return;
    };

    let mut next_dir = snake.dir;

    if keyboard.just_pressed(KeyCode::ArrowUp) && next_dir != Direction::Up {
        next_dir = Direction::Up
    };
}

fn toggle_gizmos() {

}