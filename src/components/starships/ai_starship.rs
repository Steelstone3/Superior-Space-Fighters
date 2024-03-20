use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    reflect::Reflect,
};

#[derive(Component, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct AIStarship;
