use bevy::{
    audio::AudioSink,
    ecs::query::{Or, QueryFilter, With},
    ui::Node,
};

use crate::components::{
    planet::Planet,
    space::Space,
    starships::starship::Starship,
    station::SpaceStation,
    weapons::player_weapons::{
        player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
        player_torpedo::PlayerTorpedo,
    },
};

#[derive(QueryFilter)]
pub struct InGameEntityFilter {
    pub or_filter: Or<InGameEntitiesFilter>,
}

type InGameEntitiesFilter = (
    With<Node>,
    With<Planet>,
    With<AudioSink>,
    With<PlayerBlaster>,
    With<PlayerMine>,
    With<PlayerTorpedo>,
    With<PlayerExotic>,
    With<Space>,
    With<SpaceStation>,
    With<Starship>,
);
