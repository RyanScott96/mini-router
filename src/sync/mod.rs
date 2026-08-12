use bevy::prelude::*;

use crate::sync::systems::{
    attach_packet_visuals, attach_sink_visuals, attach_source_visuals, attach_track_visuals,
    sync_transforms,
};

pub mod systems;

pub struct RoutingPlugin;
impl Plugin for RoutingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                attach_packet_visuals,
                attach_source_visuals,
                attach_sink_visuals,
                attach_track_visuals,
                sync_transforms,
            ),
        );
    }
}
