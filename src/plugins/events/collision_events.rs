use crate::events::collision_events::{
    PlayerBlasterCollisionEvent, PlayerExoticCollisionEvent, PlayerMineCollisionEvent,
    PlayerTorpedoCollisionEvent,
};
use bevy::app::Plugin;

pub struct CollisionEvents;

impl Plugin for CollisionEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerBlasterCollisionEvent>()
            .add_event::<PlayerTorpedoCollisionEvent>()
            .add_event::<PlayerMineCollisionEvent>()
            .add_event::<PlayerExoticCollisionEvent>();
    }
}
