use bevy::prelude::{App, Plugin};

use super::{combat::Combat, trading::Trading, world::World};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(World)
            .add_plugins(Trading)
            .add_plugins(Combat);
    }
}
