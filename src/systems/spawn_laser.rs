use bevy::prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, With};

use crate::{components::player::Player, resources::ammunition::Ammunition};

pub fn spawn_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<Ammunition>,
    player: Query<&Transform, With<Player>>,
) {
}
