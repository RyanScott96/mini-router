use bevy::prelude::*;

use crate::routing::RoutingPlugin;

pub mod routing;
pub mod sync;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(RoutingPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
