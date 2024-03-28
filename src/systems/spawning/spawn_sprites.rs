use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    math::{Quat, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

use crate::{
    components::{
        planet::Planet,
        starships::{player_starship::PlayerStarship, starship::Starship},
    },
    events::{
        spawn_planet_sprite_event::SpawnPlanetSpriteEvent,
        spawn_player_sprite_event::SpawnPlayerSpriteEvent,
        spawn_starship_sprite_event::SpawnStarshipSpriteEvent,
    },
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};

//TODO Find way of passing generic struct via event to remove need for multiple event handlers
pub fn spawn_sprites(
    mut ev_spawn_players: EventReader<SpawnPlayerSpriteEvent>,
    mut ev_spawn_planets: EventReader<SpawnPlanetSpriteEvent>,
    mut ev_spawn_starships: EventReader<SpawnStarshipSpriteEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_spawn_players.read() {
        let texture = asset_server.load(&ev.sprite_path);
        let player = PlayerStarship::default();

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ev.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: ev.translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(player);
    }

    for ev in ev_spawn_planets.read() {
        let texture = asset_server.load(&ev.sprite_path);
        let size = random_value_f32(generate_seed(), 25.0..500.0);
        let planet = Planet::new(random(), size);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(planet.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: ev.translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(planet);
    }

    for ev in ev_spawn_starships.read() {
        let texture = asset_server.load(&ev.sprite_path);
        let ship = Starship::new(random(), random());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ev.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: ev.translation,
                    rotation: Quat::from_axis_angle(
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        random_value_f32(generate_seed(), 0.0..360.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ship);
    }
}
