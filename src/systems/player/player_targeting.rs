use bevy::{
    ecs::{
        query::Without,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    transform::components::Transform,
};

use crate::components::{player_starship::PlayerStarship, starship::Starship};

pub fn player_targeting(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, &mut PlayerStarship), Without<Starship>>,
    mut otherShips: Query<(&Transform, &mut Starship)>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        for (transform, Starship) in &mut otherShips {
            bevy::log::info!("Target Locked");
        }
    }
}
