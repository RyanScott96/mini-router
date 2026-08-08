use std::collections::HashMap;

use bevy::prelude::*;

use crate::routing::{
    components::{GridTile, NetworkPacket, NetworkSink, NetworkSource, Track},
    messages::{CollisionMessage, SuccessPacket},
    resources::Grid,
};

pub fn advance_packets(
    mut packets: Query<&mut GridTile, With<NetworkPacket>>,
    grid: Res<Grid>,
    tracks: Query<&Track>,
) {
    for mut tile in packets.iter_mut() {
        if let Some(&track_entity) = grid.tracks.get(&*tile) {
            // Get the track component to figure out where to route
            if let Ok(track) = tracks.get(track_entity) {
                *tile = track.output.offset(*tile);
            }
        }
    }
}

pub fn spawn_packets(
    mut commands: Commands,
    mut sources: Query<(&GridTile, &mut NetworkSource)>,
    time: Res<Time>,
) {
    for (tile, mut source) in sources.iter_mut() {
        source.timer.tick(time.delta());

        if source.timer.just_finished() {
            commands.spawn((
                NetworkPacket {
                    color: source.color,
                },
                *tile,
            ));
        }
    }
}

pub fn process_network(
    mut commands: Commands,
    packets: Query<(Entity, &GridTile, &NetworkPacket)>,
    sinks: Query<&NetworkSink>,
    grid: Res<Grid>,
    mut collision_writer: MessageWriter<CollisionMessage>,
    mut delivery_writer: MessageWriter<SuccessPacket>,
) {
    let mut tile_occupancy: HashMap<GridTile, Vec<Entity>> = HashMap::default();

    for (entity, tile, _) in packets.iter() {
        tile_occupancy.entry(*tile).or_default().push(entity);
    }

    for (tile, occupying_packets) in tile_occupancy.drain() {
        if occupying_packets.len() > 1 {
            collision_writer.write(CollisionMessage {
                location: GridTile {
                    x: tile.x,
                    y: tile.y,
                },
            });
        }
    }
}
