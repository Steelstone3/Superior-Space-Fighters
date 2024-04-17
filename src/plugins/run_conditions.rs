use bevy::ecs::system::{Res, Resource};

pub fn resource_available<T: Resource>(resource: Option<Res<T>>) -> bool {
    let Some(_) = resource else {
        return false;
    };
    true
}
