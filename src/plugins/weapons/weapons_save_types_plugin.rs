use bevy::{app::Plugin, time::TimerMode};

use crate::{
    assets::{
        images::starships::weapons::{
            blasters::BlasterSprite, exotics::ExoticSprite, mines::MineSprite,
            torpedos::TorpedoSprite,
        },
        sounds::starships::weapons::{
            blasters::BlasterSound, exotics::ExoticSound, impacts::ImpactSound, mines::MineSound,
            torpedos::TorpedoSound,
        },
    },
    components::weapons::{
        ai_weapons::{blaster::Blaster, exotic::Exotic, mine::Mine, torpedo::Torpedo},
        player_weapons::{
            player_blaster::PlayerBlaster, player_exotic::PlayerExotic, player_mine::PlayerMine,
            player_torpedo::PlayerTorpedo,
        },
        weapon_types::{
            damage::Damage, lifetime_weapon::LifetimeWeapon, lock_on_weapon::LockOnWeapon,
            ranged_weapon::RangedWeapon, weapon::Weapon,
        },
    },
    resources::{
        projectile_ammunition_resource::ProjectileAmmunitionResource,
        selected_weapon::SelectedWeaponResource,
    },
};

pub struct WeaponSaveTypesPlugin;

impl Plugin for WeaponSaveTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ProjectileAmmunitionResource>();
        app.register_type::<SelectedWeaponResource>();
        app.register_type::<PlayerBlaster>();
        app.register_type::<Blaster>();
        app.register_type::<BlasterSprite>();
        app.register_type::<BlasterSound>();
        app.register_type::<ImpactSound>();
        app.register_type::<RangedWeapon>();
        app.register_type::<PlayerTorpedo>();
        app.register_type::<Torpedo>();
        app.register_type::<TorpedoSprite>();
        app.register_type::<TorpedoSound>();
        app.register_type::<PlayerMine>();
        app.register_type::<Mine>();
        app.register_type::<MineSound>();
        app.register_type::<MineSprite>();
        app.register_type::<LifetimeWeapon>();
        app.register_type::<TimerMode>();
        app.register_type::<PlayerExotic>();
        app.register_type::<Exotic>();
        app.register_type::<ExoticSound>();
        app.register_type::<ExoticSprite>();
        app.register_type::<LockOnWeapon>();
        app.register_type::<Weapon>();
        app.register_type::<Damage>();
    }
}
