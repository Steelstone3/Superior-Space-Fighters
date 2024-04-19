use bevy::{
    ecs::{query::With, world::World},
    transform::components::Transform,
    utils::tracing,
};
use bevy_save::{DefaultDebugBackend, DefaultDebugFormat, Pipeline, Snapshot, SnapshotBuilder};

use crate::{
    components::starships::{
        ai_starship::AIStarship, player_starship::PlayerStarship, starship::Starship,
    },
    events::user_interface_events::InGameUserInterfaceEvent,
    resources::projectile_ammunition_resource::ProjectileAmmunitionResource,
};

/// Debug saving impl will save files at given location in json format for easy testing
pub struct DebugSaveLoadPipeline<'a>(pub &'a str);

impl<'a> Pipeline for DebugSaveLoadPipeline<'a> {
    type Backend = DefaultDebugBackend;
    type Format = DefaultDebugFormat;

    type Key<'k> = &'k str;

    fn key(&self) -> Self::Key<'_> {
        self.0
    }

    fn capture(builder: SnapshotBuilder) -> Snapshot {
        tracing::info!("Extracting entities and resources for saving");
        builder
            .deny_all()
            .allow::<ProjectileAmmunitionResource>()
            .allow::<Starship>()
            .allow::<PlayerStarship>()
            .allow::<AIStarship>()
            .allow::<Transform>()
            .extract_entities_matching(|filter| filter.contains::<Starship>())
            .extract_resource::<ProjectileAmmunitionResource>()
            .build()
    }

    fn apply<'b>(world: &'b mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        tracing::info!("Loading entities and resources");
        //update game ui on load
        world.send_event(InGameUserInterfaceEvent {});

        let result = snapshot.applier(world).despawn::<With<Starship>>().apply();

        result
    }
}
