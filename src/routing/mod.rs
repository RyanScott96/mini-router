use bevy::prelude::*;

use crate::routing::messages::{CollisionMessage, SuccessPacket};
use crate::routing::systems::{advance_packets, process_network, spawn_packets};

pub mod components;
pub mod messages;
pub mod resources;
pub mod systems;

pub struct RoutingPlugin;
impl Plugin for RoutingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CollisionMessage>();
        app.add_message::<SuccessPacket>();
        app.add_systems(
            FixedUpdate,
            (advance_packets, spawn_packets, process_network),
        );
    }
}
