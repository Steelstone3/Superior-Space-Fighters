use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::{color::Color, texture::Image},
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, UiImage, Val},
    utils::default,
};

use crate::{
    components::user_interface::{
        weapon_selection::WeaponSelection, weapon_selection_parent::WeaponSelectionParent,
    },
    events::ui_selected_weapon_event::UISelectedWeaponEvent,
    resources::selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
};

pub fn update_weapon_selection_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    icons_root: Query<Entity, With<WeaponSelectionParent>>,
    selected_weapon: Res<SelectedWeapon>,
    mut ev_selected_weapon: EventReader<UISelectedWeaponEvent>,
) {
    //event called whenever selected weapon changes
    for _ev in ev_selected_weapon.read() {
        if let Ok(icons_root) = icons_root.get_single() {
            commands.entity(icons_root).despawn_recursive();
        }

        commands
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                    grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                    //Must set specific width + height otherwise images wont know what size to display
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    position_type: PositionType::Absolute,
                    right: Val::Percent(0.0),
                    bottom: Val::Percent(0.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            })
            .insert(WeaponSelectionParent {})
            .with_children(|parent| {
                let weapon_icon: WeaponSelection;
                if selected_weapon.selected_weapon == SelectedWeaponEnum::Blaster as u32 {
                    weapon_icon = WeaponSelection::new_selected_blaster_icon();
                } else {
                    weapon_icon = WeaponSelection::new_unselected_blaster_icon();
                }
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection;
                if selected_weapon.selected_weapon == SelectedWeaponEnum::Torpedo as u32 {
                    weapon_icon = WeaponSelection::new_selected_torpedo_icon();
                } else {
                    weapon_icon = WeaponSelection::new_unselected_torpedo_icon();
                }
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection;
                if selected_weapon.selected_weapon == SelectedWeaponEnum::Mine as u32 {
                    weapon_icon = WeaponSelection::new_selected_mine_icon();
                } else {
                    weapon_icon = WeaponSelection::new_unselected_mine_icon();
                }
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection;
                if selected_weapon.selected_weapon == SelectedWeaponEnum::Exotic as u32 {
                    weapon_icon = WeaponSelection::new_selected_exotic_icon();
                } else {
                    weapon_icon = WeaponSelection::new_unselected_exotic_icon();
                }
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);
            });
    }
}

fn weapon_icon_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
        background_color: Color::WHITE.into(),
        ..default()
    }
}
