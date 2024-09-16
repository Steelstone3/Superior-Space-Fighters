use crate::{
    states::core_states::GameState,
    systems::weapons::player_weapons::{
        player_blaster::player_blaster_collision::player_blaster_collision_with_starship,
        player_exotic::player_exotic_collision::player_exotic_collision_with_starship,
        player_mine::player_mine_collision::player_mine_collision_with_starship,
        player_torpedo::player_torpedo_collision::player_torpedo_collision_with_starship,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::in_state,
};

pub struct CollisionEventHandlersPlugin;

impl Plugin for CollisionEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                player_blaster_collision_with_starship,
                player_torpedo_collision_with_starship,
                player_mine_collision_with_starship,
                player_exotic_collision_with_starship,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
