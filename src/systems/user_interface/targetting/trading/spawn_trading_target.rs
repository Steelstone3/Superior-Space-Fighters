use crate::{
    components::{starships::starship::Starship, user_interface::targetting::target::Target},
    events::spawn_sprite_event::SpawnSpriteEvent,
    resources::targetting_settings::TargettingSettings,
};
use bevy::{
    ecs::{event::EventWriter, query::With, system::ResMut},
    input::ButtonInput,
    math::Quat,
    prelude::{Commands, KeyCode, Query, Res, Transform},
    utils::tracing,
};

pub fn spawn_trading_target(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_setting: ResMut<TargettingSettings>,
    starship_transforms: Query<&Transform, With<Starship>>,
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
) {
    if !input.just_pressed(KeyCode::KeyH) {
        return;
    }

    let mut random_starship_transform = None;

    for starship_transform in starship_transforms.into_iter() {
        random_starship_transform = Some(starship_transform)
    }

    if let Some(random_starship) = random_starship_transform {
        if !targetting_setting.is_targetting {
            let target = Target::create_trading_target();
            let texture = target.lock_on_target.to_string();
            let size = target.lock_on_target_size;
            tracing::info!("Spawning Trading Target");

            let entity = commands.spawn(target).id();

            let event = SpawnSpriteEvent {
                sprite_path: texture,
                size,
                translation: random_starship.translation,
                entity,
                rotation: Quat::default(),
            };
            ev_spawn_sprite.send(event);
        }
    }

    targetting_setting.is_targetting = true;
}
