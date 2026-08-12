use bevy::prelude::*;

use crate::routing::components::{
    GridTile, NetworkPacket, NetworkSink, NetworkSource, PacketColor, Track,
};

const TILE_SIZE: f32 = 64.0;

fn grid_to_world(tile: &GridTile, z_index: f32) -> Vec3 {
    Vec3::new(
        tile.x as f32 * TILE_SIZE,
        tile.y as f32 * TILE_SIZE,
        z_index,
    )
}

pub fn attach_packet_visuals(
    mut commands: Commands,
    query: Query<(Entity, &GridTile, &NetworkPacket), Added<NetworkPacket>>,
) {
    for (entity, tile, packet) in query.iter() {
        let visual_color = match packet.color {
            PacketColor::Red => Color::srgba(1.0, 0.2, 0.2, 1.0),
            PacketColor::Blue => Color::srgba(0.2, 0.2, 1.0, 1.0),
            PacketColor::Green => Color::srgba(0.2, 1.0, 0.2, 1.0),
        };

        commands.entity(entity).insert((
            Sprite {
                color: visual_color,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.4, TILE_SIZE * 0.4)),
                ..default()
            },
            Transform::from_translation(grid_to_world(tile, 1.0)),
        ));
    }
}

pub fn attach_source_visuals(
    mut commands: Commands,
    query: Query<(Entity, &GridTile, &NetworkSource), Added<NetworkSource>>,
) {
    for (entity, tile, source) in query.iter() {
        let visual_color = match source.color {
            PacketColor::Red => Color::srgba(1.0, 0.2, 0.2, 1.0),
            PacketColor::Blue => Color::srgba(0.2, 0.2, 1.0, 1.0),
            PacketColor::Green => Color::srgba(0.2, 1.0, 0.2, 1.0),
        };

        commands.entity(entity).insert((
            Sprite {
                color: visual_color,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_translation(grid_to_world(tile, 0.5)),
        ));
    }
}

pub fn attach_sink_visuals(
    mut commands: Commands,
    query: Query<(Entity, &GridTile, &NetworkSink), Added<NetworkSink>>,
) {
    for (entity, tile, sink) in query.iter() {
        let visual_color = match sink.color {
            PacketColor::Red => Color::srgba(1.0, 0.2, 0.2, 1.0),
            PacketColor::Blue => Color::srgba(0.2, 0.2, 1.0, 1.0),
            PacketColor::Green => Color::srgba(0.2, 1.0, 0.2, 1.0),
        };

        commands.entity(entity).insert((
            Sprite {
                color: visual_color,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                ..default()
            },
            Transform::from_translation(grid_to_world(tile, 0.5)),
        ));
    }
}

pub fn attach_track_visuals(
    mut commands: Commands,
    query: Query<(Entity, &GridTile, &Track), Added<Track>>,
) {
    for (entity, tile, _) in query.iter() {
        commands.entity(entity).insert((
            Sprite {
                color: Color::srgba(0.3, 0.3, 0.3, 1.0),
                custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                ..default()
            },
            Transform::from_translation(grid_to_world(tile, 0.0)),
        ));
    }
}

pub fn sync_transforms(
    // Only fetch entities where the GridTile was mutated since last frame
    mut query: Query<(&GridTile, &mut Transform), Changed<GridTile>>,
) {
    for (tile, mut transform) in query.iter_mut() {
        transform.translation = grid_to_world(tile, transform.translation.z);
    }
}
