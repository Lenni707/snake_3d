use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod camera;
mod grid;
mod snake;
mod world;
mod controls;

use crate::camera::CameraPlugin;
use crate::grid::Grid;
use crate::snake::SnakePlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Grid::new())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins((SnakePlugin, CameraPlugin, WorldPlugin))
        .run();
}
