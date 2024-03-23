use crate::{
    components::{
        starships::player_starship::PlayerStarship,
        weapons::player_weapons::player_blaster::PlayerBlaster,
    },
    events::spawn_sprite_event::SpawnSpriteEvent,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::{
    ecs::event::EventWriter,
    input::ButtonInput,
    math::Vec3,
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    utils::tracing,
};

pub fn spawn_player_blaster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    weapon_selection: Res<SelectedWeapon>,
    player_query: Query<&Transform, With<PlayerStarship>>,
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
) {
    if weapon_selection.selected_weapon != SelectedWeaponEnum::Blaster as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.blaster_ammunition < 1 {
        tracing::info!("Out of blaster ammunition");
        return;
    }

    let mut player_transform = *player_query.get_single().unwrap();
    player_transform.translation.z = 3.0;

    let blaster = PlayerBlaster::new(Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ));

    let image_path = blaster.blaster.blaster.to_string();
    let sound_path = blaster.blaster.firing_sound.to_string();
    let size = blaster.blaster.ranged_weapon.weapon.size;
    let entity = commands.spawn(blaster).id();

    let event = SpawnSpriteEvent {
        sprite_path: image_path,
        size,
        translation: player_transform.translation,
        entity,
        rotation: player_transform.rotation,
    };

    ev_spawn_sprite.send(event);

    commands.spawn(AudioBundle {
        source: asset_server.load(sound_path),
        ..Default::default()
    });

    ammunition.blaster_ammunition -= 1;
    tracing::info!(
        "Fired 1 blaster shot. {:?} blaster shots remaining",
        ammunition.blaster_ammunition
    );
}
