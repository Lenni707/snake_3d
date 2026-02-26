use bevy::prelude::*;

mod grid;
mod snake;
mod camera;

use crate::grid::Grid;
use crate::snake::SnakePlugin;
use crate::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Grid::new())
        .add_plugins((SnakePlugin, CameraPlugin))
        .run();
}