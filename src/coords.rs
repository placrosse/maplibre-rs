use crate::render::shader_ffi::Vec3f32;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct TileCoords {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl TileCoords {
    pub fn into_world_tile(self) -> WorldTileCoords {
        WorldTileCoords {
            x: self.x as i32 - crate::example::MUNICH_X as i32,
            y: (self.y as i32 - crate::example::MUNICH_Y as i32 + 1) * -1,
            z: 0,
        }
    }
}

impl fmt::Display for TileCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T({x}, {y}, {z})", x = self.x, y = self.y, z = self.z)
    }
}

impl From<(u32, u32, u8)> for TileCoords {
    fn from(tuple: (u32, u32, u8)) -> Self {
        TileCoords {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorldTileCoords {
    pub x: i32,
    pub y: i32,
    pub z: u8,
}

impl WorldTileCoords {
    pub fn into_world(self, extent: u16) -> WorldCoords {
        WorldCoords {
            x: self.x as f32 * extent as f32,
            y: self.y as f32 * extent as f32 + extent as f32, // We add extent here as we want the upper left corner
            z: self.z as f32,
        }
    }

    pub fn stencil_reference_value(&self) -> u8 {
        match (self.x, self.y) {
            (x, y) if x % 2 == 0 && y % 2 == 0 => 1,
            (x, y) if x % 2 == 0 && y % 2 != 0 => 2,
            (x, y) if x % 2 != 0 && y % 2 == 0 => 3,
            (x, y) if x % 2 != 0 && y % 2 != 0 => 4,
            _ => 0,
        }
    }
}

impl fmt::Display for WorldTileCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WT({x}, {y}, {z})", x = self.x, y = self.y, z = self.z)
    }
}

impl From<(i32, i32, u8)> for WorldTileCoords {
    fn from(tuple: (i32, i32, u8)) -> Self {
        WorldTileCoords {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorldCoords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldCoords {
    pub fn into_shader_coords(self) -> Vec3f32 {
        [self.x, self.y, self.z]
    }
}

impl fmt::Display for WorldCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "W({x}, {y}, {z})", x = self.x, y = self.y, z = self.z)
    }
}

impl From<(f32, f32, f32)> for WorldCoords {
    fn from(tuple: (f32, f32, f32)) -> Self {
        WorldCoords {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}
