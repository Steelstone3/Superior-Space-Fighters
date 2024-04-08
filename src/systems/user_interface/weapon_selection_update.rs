use crate::{
    components::user_interface::{
        ammo_count_parent::AmmoCountParent, weapon_selection::WeaponSelection,
        weapon_selection_parent::WeaponSelectionParent,
    },
    events::user_interface_events::UserInterfaceEvent,
    queries::weapon_selection_parent_queries::WeaponSelectionParentEntityQuery,
    resources::selected_weapon::{SelectedWeapon, SelectedWeaponEnum},
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::{color::Color, texture::Image},
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
    utils::{default, tracing},
};

pub fn update_weapon_selection_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weapon_selection_parents: Query<WeaponSelectionParentEntityQuery>,
    selected_weapon: Res<SelectedWeapon>,
    mut user_interface_event: EventReader<UserInterfaceEvent>,
) {
    //event called whenever selected weapon changes
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
                    .insert(weapon_icon);
                parent
                    .spawn(weapon_ammo_count_bundle())
                    .insert(AmmoCountParent {});

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Torpedo as u32 {
                        WeaponSelection::new_selected_torpedo_icon()
                    } else {
                        WeaponSelection::new_unselected_torpedo_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);
                parent
                    .spawn(weapon_ammo_count_bundle())
                    .insert(AmmoCountParent {});

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Mine as u32 {
                        WeaponSelection::new_selected_mine_icon()
                    } else {
                        WeaponSelection::new_unselected_mine_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);
                parent
                    .spawn(weapon_ammo_count_bundle())
                    .insert(AmmoCountParent {});

                let weapon_icon: WeaponSelection =
                    if selected_weapon.selected_weapon == SelectedWeaponEnum::Exotic as u32 {
                        WeaponSelection::new_selected_exotic_icon()
                    } else {
                        WeaponSelection::new_unselected_exotic_icon()
                    };
                let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
                parent
                    .spawn((weapon_icon_node_bundle(), UiImage::new(texture)))
                    .insert(weapon_icon);
                parent
                    .spawn(weapon_ammo_count_bundle())
                    .insert(AmmoCountParent {});
            });
    }
}

fn weapon_icon_parent_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::flex(1.0),
                GridTrack::flex(0.5),
                GridTrack::flex(1.0),
                GridTrack::flex(0.5),
            ],
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
            //Must set specific width + height otherwise images wont know what size to display
            width: Val::Px(150.0),
            height: Val::Px(100.0),
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
            ..default()
        },
        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
        background_color: Color::WHITE.into(),
        ..default()
    }
}

fn weapon_ammo_count_bundle() -> TextBundle {
    TextBundle::from_section(
        "1",
        TextStyle {
            font_size: 30.0,
            ..Default::default()
        },
    )
    .with_style(Style {
        align_self: bevy::ui::AlignSelf::Center,
        margin: UiRect::all(Val::Auto),
        ..Default::default()
    })
}
