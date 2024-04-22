use bevy::ecs::{
    event::{Event, EventReader},
    system::{Res, Resource},
};

pub fn resource_available<T: Resource>(resource: Option<Res<T>>) -> bool {
    let Some(_) = resource else {
        return false;
    };
    true
}

pub fn event_called<T: Event>(mut event_reader: EventReader<T>) -> bool {
    for _ in event_reader.read() {
        return true;
    }
    return false;
}
