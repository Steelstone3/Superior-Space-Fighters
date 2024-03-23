use crate::{
    components::{
        starships::player_starship::PlayerStarship,
        weapons::player_weapons::player_mine::PlayerMine,
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
    prelude::{AssetServer, AudioBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With},
    utils::tracing,
};

pub fn spawn_player_mine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut ammunition: ResMut<ProjectileAmmunition>,
    weapon_selection: Res<SelectedWeapon>,
    player: Query<&Transform, With<PlayerStarship>>,
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
) {
    if weapon_selection.selected_weapon != SelectedWeaponEnum::Mine as u32 {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if ammunition.mine_ammunition < 1 {
        tracing::info!("Out of mines");
        return;
    }

    let mut player_transform = *player.get_single().unwrap();
    let mine_size = 100.0;

    let mine_spawn_position =
        player_transform.translation + player_transform.down() * (mine_size / 1.75);
    player_transform.translation = mine_spawn_position;
    player_transform.translation.z = 3.0;

    let mine = PlayerMine::default();

    let image_path = mine.mine.mine.to_string();
    let sound_path = mine.mine.firing_sound.to_string();
    let size = mine.mine.lifetime_weapon.weapon.size;
    let entity = commands.spawn(mine).id();

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

    ammunition.mine_ammunition -= 1;
    tracing::info!(
        "Fired 1 mine. {:?} mines remaining",
        ammunition.mine_ammunition
    );
}
