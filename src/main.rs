use bevy::prelude::*;

use crate::routing::RoutingPlugin;

pub mod routing;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RoutingPlugin)
        .run();
}
