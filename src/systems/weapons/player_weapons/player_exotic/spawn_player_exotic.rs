use crate::{
    components::{
        starships::player_starship::PlayerStarship,
        weapons::player_weapons::player_exotic::PlayerExotic,
    },
    events::spawn_sprite_event::SpawnSpriteEvent,
    resources::{
        projectile_ammunition::ProjectileAmmunition,
        selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
    },
};
use bevy::math::Vec3;
use bevy::{ecs::event::EventWriter, input::ButtonInput};
use bevy::{
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    utils::tracing,
};

pub fn spawn_player_exotic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    selected_weapon: Res<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
) {
    if selected_weapon.selected_weapon != SelectedWeaponEnum::Exotic as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.exotic_ammunition < 1 {
        tracing::info!("Out of exotic ammunition");
        return;
    }

    let mut player_transform = *player.get_single().unwrap();
    player_transform.translation.z = 3.0;

    let exotic = PlayerExotic::new(Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z,
    ));

    let image_path = exotic.exotic.exotic.to_string();
    let sound_path = exotic.exotic.firing_sound.to_string();
    let size = exotic.exotic.ranged_weapon.weapon.size;
    let entity = commands.spawn(exotic).id();

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

    ammunition.exotic_ammunition -= 1;
    tracing::info!(
        "Fired 1 exotic shot. {:?} exotic shots remaining",
        ammunition.exotic_ammunition
    );
}
