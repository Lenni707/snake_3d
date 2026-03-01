use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub size: i32,
    pub dimensions: IVec3,
    pub origin: Vec3,
    pub cell_size: f32,
}

const GRID_SIZE: i32 = 15;

impl Grid {
    pub fn new() -> Self {
        Grid {
            size: GRID_SIZE,
            dimensions: IVec3::splat(GRID_SIZE),
            origin: Vec3::new(0.0, 0.0, 0.0),
            cell_size: 0.5,
        }
    }
    pub fn in_bounds(&self, c: IVec3) -> bool {
        c.x >= self.origin.x as i32
            && c.y >= self.origin.y as i32
            && c.z >= self.origin.z as i32
            && c.x < self.dimensions.x
            && c.y < self.dimensions.y
            && c.z < self.dimensions.z
    }
    pub fn cell_to_world(&self, c: IVec3) -> Vec3 {
        // gets the center of each cell in the grid
        self.origin + (c.as_vec3() + Vec3::splat(0.5)) * self.cell_size
    }
}
