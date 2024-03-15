use super::ranged_weapon::RangedWeapon;
use crate::{
    assets::images::starships::weapons::targetting::Targetting,
    components::starships::starship::Starship,
};
use bevy::{ecs::component::Component, transform::components::Transform};

// TODO idea for targetting system
#[derive(Component, Debug, PartialEq)]
pub struct LockOnWeapon {
    pub lock_on_target: Targetting,
    pub starship_target: (Transform, Starship),
    pub weapon: RangedWeapon,
}

// impl RangedWeapon {
//     pub fn new(original_position: Vec3, size: f32, velocity: f32, range: f32) -> Self {
//         Self {
//             range,
//             original_position,
//             weapon: Weapon::new(size, velocity),
//         }
//     }
// }

// #[cfg(test)]
// mod lock_on_weapon_should {
//     use bevy::math::Vec2;

//     use crate::components::weapons::weapon_types::damage::Damage;

//     use super::*;
//     #[test]
//     fn create_weapon() {
//         // Given
//         let original_position = Vec3 {
//             x: 1.0,
//             y: 2.0,
//             z: 3.0,
//         };
//         let velocity = 100.0;
//         let size = 100.0;
//         let range = 750.0;
//         let expected_ranged_weapon = RangedWeapon {
//             range,
//             original_position,
//             weapon: Weapon {
//                 velocity,
//                 size: Vec2 { x: size, y: size },
//                 damage: Damage {
//                     base_damage: 10,
//                     damage: Default::default(),
//                 },
//             },
//         };

//         // When
//         let ranged_weapon = RangedWeapon::new(original_position, size, velocity, range);

//         // Then
//         assert_eq!(expected_ranged_weapon, ranged_weapon);
//     }
// }
