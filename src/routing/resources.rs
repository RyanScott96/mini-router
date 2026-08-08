use std::collections::HashMap;

use bevy::prelude::*;

use crate::routing::components::GridTile;

#[derive(Resource, Debug, Default)]
pub struct Grid {
    pub sources: HashMap<GridTile, Entity>,
    pub sinks: HashMap<GridTile, Entity>,
    pub tracks: HashMap<GridTile, Entity>,
}
