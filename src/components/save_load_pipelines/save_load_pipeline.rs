// TODO Fix system

// use bevy::{
//     ecs::{
//         query::{Or, With},
//         world::World,
//     },
//     transform::components::Transform,
//     utils::tracing,
// };
// use bevy_save::{DefaultBackend, DefaultFormat, Pipeline, Snapshot, SnapshotBuilder};

// use crate::{
//     components::{
//         space::Space,
//         starships::{ai_starship::AIStarship, player_starship::PlayerStarship, starship::Starship},
//         station::SpaceStation,
//         weapons::player_weapons::{
//             player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
//             player_torpedo::PlayerTorpedo,
//         },
//     },
//     events::user_interface_events::{InGameUserInterfaceEvent, PauseEvent},
//     resources::{
//         projectile_ammunition_resource::ProjectileAmmunitionResource,
//         selected_weapon::SelectedWeaponResource,
//     },
// };

// pub struct SaveLoadPipeline<'a>(pub &'a str);

// impl<'a> Pipeline for SaveLoadPipeline<'a> {
//     type Backend = DefaultBackend;
//     type Format = DefaultFormat;

//     type Key<'k> = &'k str;

//     fn key(&self) -> Self::Key<'_> {
//         self.0
//     }

//     fn capture(builder: SnapshotBuilder) -> Snapshot {
//         tracing::info!("Extracting entities and resources for saving");
//         builder
//             //this is the list of things that we do want to save
//             .allow::<ProjectileAmmunitionResource>()
//             .allow::<SelectedWeaponResource>()
//             .allow::<Starship>()
//             .allow::<PlayerStarship>()
//             .allow::<AIStarship>()
//             .allow::<Transform>()
//             .allow::<Space>()
//             .allow::<SpaceStation>()
//             .allow::<PlayerBlaster>()
//             .allow::<PlayerTorpedo>()
//             .allow::<PlayerMine>()
//             .allow::<PlayerExotic>()
//             //save objects with components matching filter
//             .extract_entities_matching(|filter| {
//                 filter.contains::<Starship>()
//                     || filter.contains::<Space>()
//                     || filter.contains::<SpaceStation>()
//                     || filter.contains::<PlayerBlaster>()
//                     || filter.contains::<PlayerMine>()
//                     || filter.contains::<PlayerTorpedo>()
//                     || filter.contains::<PlayerExotic>()
//             })
//             //list of resources to save
//             .extract_resource::<ProjectileAmmunitionResource>()
//             .extract_resource::<SelectedWeaponResource>()
//             .build()
//     }

//     fn apply(world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
//         tracing::info!("Loading entities and resources");
//         //update game ui on load
//         world.send_event(InGameUserInterfaceEvent {});
//         //unpause game once loaded
//         world.send_event(PauseEvent { pause: false });

//         let result = snapshot
//             .applier(world)
//             //despawn any existing types that we have saved before reloading them
//             .despawn::<Or<(
//                 With<Starship>,
//                 With<Space>,
//                 With<SpaceStation>,
//                 With<PlayerBlaster>,
//                 With<PlayerMine>,
//                 With<PlayerTorpedo>,
//                 With<PlayerExotic>,
//             )>>()
//             .apply();
//         result
//     }
// }
