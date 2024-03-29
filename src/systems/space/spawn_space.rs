use crate::{
    assets::images::space::SpaceSprite, components::space::Space,
    events::spawn_sprite_event::SpawnSpriteEvent,
};
use bevy::{
    ecs::event::EventWriter,
    math::{Vec2, Vec3},
    prelude::Commands,
    transform::components::Transform,
};
use rand::random;

pub fn spawn_random_empty_space_background(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
) {
    let space = random::<SpaceSprite>();
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

            spawn_sprite_event.send(SpawnSpriteEvent {
                sprite_path: space.space.to_string(),
                size: space.size,
                entity: commands.spawn(space).id(),
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: location.z,
                    },
                    ..Default::default()
                },
            });
        }
    }
}
