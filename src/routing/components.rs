use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridTile {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Track {
    pub input: Direction,
    pub output: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    pub fn offset(&self, tile: GridTile) -> GridTile {
        match self {
            Direction::UP => GridTile {
                x: tile.x,
                y: tile.y + 1,
            },
            Direction::DOWN => GridTile {
                x: tile.x,
                y: tile.y - 1,
            },
            Direction::LEFT => GridTile {
                x: tile.x - 1,
                y: tile.y,
            },
            Direction::RIGHT => GridTile {
                x: tile.x + 1,
                y: tile.y,
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPacket {
    pub color: PacketColor,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct NetworkSource {
    pub color: PacketColor,
    pub timer: Timer,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkSink {
    pub color: PacketColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketColor {
    Red,
    Green,
    Blue,
}
