use crate::{
    queries::{starship_queries::StarshipQuery, target_queries::TargetQuery},
    resources::targetting_settings::TargettingSettings,
};
use bevy::{
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

pub fn despawn_combat_target(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut targetting_setting: ResMut<TargettingSettings>,
    targets: Query<TargetQuery>,
    starships: Query<StarshipQuery>,
) {
    let Ok(target) = targets.get_single() else {
        return;
    };

    if input.just_pressed(KeyCode::Tab) {
        commands.entity(target.entity).despawn();

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
            commands.entity(target.entity).despawn();
            targetting_setting.is_targetting = false;
            tracing::info!("Despawning Combat Target: No Targets Exist");
        }
    }
}
