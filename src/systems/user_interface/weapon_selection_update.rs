use crate::{
    components::user_interface::{
        weapon_selection::WeaponSelection, weapon_selection_parent::WeaponSelectionParent,
    },
    events::user_interface_events::InGameUserInterfaceEvent,
    queries::{entity_query::EntityQuery, weapon_ui_queries::WeaponUiUpdateFilter},
    resources::{
        projectile_ammunition_resource::ProjectileAmmunitionResource,
        selected_weapon::{SelectedWeaponEnum, SelectedWeaponResource},
    },
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::{color::Color, texture::Image},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        BackgroundColor, Display, GridTrack, JustifyContent, PositionType, Style, UiImage, UiRect,
        Val,
    },
    utils::default,
};

pub fn weapon_selection_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weapon_selection_parents: Query<EntityQuery, WeaponUiUpdateFilter>,
    selected_weapon: Res<SelectedWeaponResource>,
    mut user_interface_event: EventReader<InGameUserInterfaceEvent>,
    ammunition: ResMut<ProjectileAmmunitionResource>,
) {
    //event called whenever ui updates
    for _ in user_interface_event.read() {
        if let Ok(weapon_selection_parent) = weapon_selection_parents.get_single() {
            commands
                .entity(weapon_selection_parent.entity)
                .despawn_recursive();
        }

        commands
            .spawn(weapon_icon_parent_node_bundle())
            .insert(WeaponSelectionParent {})
            .with_children(|parent| {
                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Blaster as u32 {
                        WeaponSelection::new_selected_blaster_icon()
                    } else {
                        WeaponSelection::new_unselected_blaster_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .with_children(|blaster_parent| {
                        blaster_parent
                            .spawn(weapon_ammo_count_bundle(ammunition.blaster_ammunition));
                    })
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Torpedo as u32 {
                        WeaponSelection::new_selected_torpedo_icon()
                    } else {
                        WeaponSelection::new_unselected_torpedo_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .with_children(|blaster_parent| {
                        blaster_parent
                            .spawn(weapon_ammo_count_bundle(ammunition.torpedo_ammunition));
                    })
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Mine as u32 {
                        WeaponSelection::new_selected_mine_icon()
                    } else {
                        WeaponSelection::new_unselected_mine_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .with_children(|blaster_parent| {
                        blaster_parent.spawn(weapon_ammo_count_bundle(ammunition.mine_ammunition));
                    })
                    .insert(weapon_icon);

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Exotic as u32 {
                        WeaponSelection::new_selected_exotic_icon()
                    } else {
                        WeaponSelection::new_unselected_exotic_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .with_children(|blaster_parent| {
                        blaster_parent
                            .spawn(weapon_ammo_count_bundle(ammunition.exotic_ammunition));
                    })
                    .insert(weapon_icon);
            });
    }
}

fn weapon_icon_parent_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
            //Must set specific width + height otherwise images wont know what size to display
            width: Val::Px(150.0),
            height: Val::Px(150.0),
            position_type: PositionType::Absolute,
            right: Val::Percent(0.0),
            bottom: Val::Percent(0.0),
            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
        ..default()
    }
}

fn weapon_icon_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Percent(10.0)),
            justify_content: JustifyContent::End,
            ..default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..default()
    }
}

fn weapon_ammo_count_bundle(ammo_count: u32) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            ammo_count.to_string(),
            TextStyle {
                font_size: 20.0,
                color: Color::ANTIQUE_WHITE,
                ..default()
            },
        ),
        ..Default::default()
    }
}
