use bevy::{
    ecs::query::{QueryFilter, Without},
    sprite::Sprite,
};

#[derive(QueryFilter)]
pub struct NoSpriteFilter {
    no_sprite: Without<Sprite>,
}
