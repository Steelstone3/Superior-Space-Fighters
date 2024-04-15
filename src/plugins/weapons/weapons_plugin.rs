use crate::{
    plugins::run_conditions::run_if_not_paused,
    resources::{
        projectile_ammunition_recharge::ProjectileAmmunitionRecharge,
        projectile_ammunition_resource::ProjectileAmmunitionResource,
        selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
    },
    systems::{
        player::player_weapon_select::player_weapon_select,
        weapons::player_weapons::{
            ammunition_recharge::ammunition_recharge,
            player_blaster::{
                player_blaster_ammunition_consumption::player_blaster_ammunition_consumption,
                player_blaster_lifetime::player_blaster_lifetime,
                player_blaster_movement::player_blaster_movement,
                spawn_player_blaster_sprite::spawn_player_blaster_sprite,
            },
            player_exotic::{
                player_exotic_ammunition_consumption::player_exotic_ammunition_consumption,
                player_exotic_lifetime::player_exotic_lifetime,
                player_exotic_movement::player_exotic_movement,
                spawn_player_exotic_sprite::spawn_player_exotic_sprite,
            },
            player_mine::{
                player_mine_ammunition_consumption::player_mine_ammunition_consumption,
                player_mine_lifetime::player_mine_lifetime,
                player_mine_movement::player_mine_movement,
                spawn_player_mine_sprite::spawn_player_mine_sprite,
            },
            player_torpedo::{
                player_torpedo_ammunition_consumption::player_torpedo_ammunition_consumption,
                player_torpedo_lifetime::player_torpedo_lifetime,
                player_torpedo_movement::player_torpedo_movement,
                spawn_player_torpedo_sprite::spawn_player_torpedo_sprite,
            },
        },
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    time::{Timer, TimerMode},
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_weapon_select.run_if(run_if_not_paused))
            .add_systems(
                Update,
                (
                    spawn_player_blaster_sprite,
                    player_blaster_ammunition_consumption,
                    player_blaster_lifetime,
                    player_blaster_movement,
                    spawn_player_torpedo_sprite,
                    player_torpedo_ammunition_consumption,
                    player_torpedo_lifetime,
                    player_torpedo_movement,
                    spawn_player_mine_sprite,
                    player_mine_ammunition_consumption,
                    player_mine_lifetime,
                    player_mine_movement,
                    spawn_player_exotic_sprite,
                    player_exotic_ammunition_consumption,
                    player_exotic_lifetime,
                    player_exotic_movement,
                    ammunition_recharge,
                )
                    .run_if(run_if_not_paused),
            )
            .insert_resource(ProjectileAmmunitionResource {
                blaster_ammunition: 20,
                maximum_blaster_ammunition: 20,
                torpedo_ammunition: 5,
                maximum_torpedo_ammunition: 5,
                mine_ammunition: 7,
                maximum_mine_ammunition: 7,
                exotic_ammunition: 2,
                maximum_exotic_ammunition: 2,
            })
            .insert_resource(SelectedWeaponResource {
                selected_weapon: SelectedWeaponEnum::Blaster as u32,
            })
            .insert_resource(ProjectileAmmunitionRecharge {
                blaster_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
                torpedo_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
                mine_recovery_rate: Timer::from_seconds(5.0, TimerMode::Repeating),
                exotic_recovery_rate: Timer::from_seconds(15.0, TimerMode::Repeating),
            });
    }
}
