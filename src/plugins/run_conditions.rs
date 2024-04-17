use bevy::ecs::system::{Res, Resource};

pub fn run_if_resource_available<T: Resource>(resource: Option<Res<T>>) -> bool {
    let Some(_) = resource else {
        return false;
    };
    true
}
