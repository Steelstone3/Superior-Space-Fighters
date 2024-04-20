use bevy::{
    ecs::{
        query::{Or, With},
        world::World,
    },
    transform::components::Transform,
    utils::tracing,
};
use bevy_save::{DefaultDebugBackend, DefaultDebugFormat, Pipeline, Snapshot, SnapshotBuilder};

use crate::{
    components::{
        space::Space,
        starships::{ai_starship::AIStarship, player_starship::PlayerStarship, starship::Starship},
        station::SpaceStation,
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
            //most stuff like built in bevy types do not need saving so exclude them
            .deny_all()
            //this is the list of things that we do want to save
            .allow::<ProjectileAmmunitionResource>()
            .allow::<Starship>()
            .allow::<PlayerStarship>()
            .allow::<AIStarship>()
            .allow::<Transform>()
            .allow::<Space>()
            .allow::<SpaceStation>()
            //save objects with components matching filter
            .extract_entities_matching(|filter| {
                filter.contains::<Starship>()
                    || filter.contains::<Space>()
                    || filter.contains::<SpaceStation>()
            })
            //list of resources to save
            .extract_resource::<ProjectileAmmunitionResource>()
            .build()
    }

    fn apply<'b>(world: &'b mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        tracing::info!("Loading entities and resources");
        //update game ui on load
        world.send_event(InGameUserInterfaceEvent {});

        let result = snapshot
            .applier(world)
            //despawn any existing types that we have saved before reloading them
            .despawn::<Or<(With<Starship>, With<Space>)>>()
            .apply();
        result
    }
}
