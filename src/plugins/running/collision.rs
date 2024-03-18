use crate::systems::weapons::{
    player_blaster::player_blaster_collision::player_blaster_collision_with_starship,
    player_exotic::player_exotic_collision::player_exotic_collision_with_starship,
    player_mine::player_mine_collision::player_mine_collision_with_starship,
    player_torpedo::player_torpedo_collision::player_torpedo_collision_with_starship,
};
use bevy::app::{Plugin, Update};

pub struct Collision;

impl Plugin for Collision {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_blaster_collision_with_starship)
            .add_systems(Update, player_torpedo_collision_with_starship)
            .add_systems(Update, player_mine_collision_with_starship)
            .add_systems(Update, player_exotic_collision_with_starship);
    }
}
