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
    pub or_filter: Or<(With<Node>, With<Planet>, With<AudioSink>)>,
    pub or_with_weapons: Or<(With<PlayerBlaster>, With<PlayerMine>, With<PlayerTorpedo>)>,
    pub or_with_exotic: Or<(With<PlayerExotic>,)>,
    pub or_with_space: Or<(With<Space>, With<SpaceStation>, With<Starship>)>,
}
