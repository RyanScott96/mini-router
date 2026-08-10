use bevy::prelude::*;

use crate::routing::components::{GridTile, PacketColor};

#[derive(Message)]
pub struct SuccessPacket {
    pub color: PacketColor,
    pub location: GridTile,
}

#[derive(Message)]
pub struct ErrorPacket {
    pub actual: PacketColor,
    pub expected: PacketColor,
    pub location: GridTile,
}

#[derive(Message)]
pub struct CollisionMessage {
    pub location: GridTile,
}
