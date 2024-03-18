use crate::{
    components::{
        starships::starship::Starship, targetting::target::Target,
    },
    resources::targetting_settings::TargettingSettings,
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

pub fn despawn_combat_target(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_setting: ResMut<TargettingSettings>,
    targets: Query<(Entity, &Target)>,
    starships: Query<&Starship>,
) {
    let Ok(target) = targets.get_single() else {
        return;
    };

    if input.just_pressed(KeyCode::Tab) {
        commands.entity(target.0).despawn();

        targetting_setting.is_targetting = false;

        tracing::info!("Despawning Combat Target: Cancel Target Key Pressed");

        return;
    }

    let mut random_starship = None;

    for starship in starships.into_iter() {
        random_starship = Some(starship)
    }

    match random_starship {
        Some(_) => {}
        None => {
            commands.entity(target.0).despawn();
            targetting_setting.is_targetting = false;
            tracing::info!("Despawning Combat Target: No Targets Exist");
        }
    }
}
