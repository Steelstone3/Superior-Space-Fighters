use crate::{
    assets::images::space::SpaceSprite, components::space::Space,
    events::spawn_sprite_event::SpawnSpriteEvent,
};
use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::{Quat, Vec2, Vec3},
};

pub fn spawn_random_empty_space_background(
    mut ev_spawn_space: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    let space = SpaceSprite::generate_random();
    let tile_size = 1920.0;

    for x in 0..5 {
        for y in 0..5 {
            let grid_position = Vec2::new(x as f32, y as f32);
            let location = Vec3::new(
                (grid_position.x * tile_size) - tile_size,
                (grid_position.y * tile_size) - tile_size,
                0.0,
            );

            let space = Space::new(space, tile_size, grid_position, location);
            let texture = space.space.to_string();
            let size = space.size;
            let entity = commands.spawn(space).id();

            let space_event = SpawnSpriteEvent {
                sprite_path: texture,
                size,
                translation: location,
                entity,
                rotation: Quat::default(),
            };
            ev_spawn_space.send(space_event);
        }
    }
}
