use std::collections::HashMap;

use bevy::prelude::*;

use crate::routing::{
    components::{GridTile, NetworkPacket, NetworkSink, NetworkSource, Track},
    messages::{CollisionMessage, ErrorPacket, SuccessPacket},
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
    mut mismatch_writer: MessageWriter<ErrorPacket>,
) {
    let mut tile_occupancy: HashMap<GridTile, Vec<Entity>> = HashMap::default();

    for (entity, tile, _) in packets.iter() {
        tile_occupancy.entry(*tile).or_default().push(entity);
    }

    for (tile, occupying_packets) in tile_occupancy.drain() {
        // If two packets are on the same tile
        // then they are colliding and need despawned
        if occupying_packets.len() > 1 {
            collision_writer.write(CollisionMessage {
                location: GridTile {
                    x: tile.x,
                    y: tile.y,
                },
            });

            for entity in occupying_packets {
                commands.entity(entity).despawn();
            }

            continue;
        }

        let packet_entity = occupying_packets[0];

        // If a packet is on a network sink
        // then it is delievered and needs despawned
        if let Ok((_, _, packet_data)) = packets.get(packet_entity) {
            if let Some(&sink_entity) = grid.sinks.get(&tile) {
                if let Ok(sink) = sinks.get(sink_entity) {
                    // Packet is Good if same color
                    if sink.color == packet_data.color {
                        delivery_writer.write(SuccessPacket {
                            color: sink.color,
                            location: tile,
                        });
                    }
                    // Packet is Bad if diff color
                    else {
                        mismatch_writer.write(ErrorPacket {
                            expected: sink.color,
                            actual: packet_data.color,
                            location: tile,
                        });
                    }

                    commands.entity(packet_entity).despawn();
                }
            }
        }
    }
}
