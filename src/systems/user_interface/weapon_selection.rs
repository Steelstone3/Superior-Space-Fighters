use bevy::{
    asset::{AssetServer, Handle},
    color::Color,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    prelude::{default, NodeBundle},
    render::texture::Image,
    ui::{Display, GridTrack, PositionType, Style, UiImage, Val},
};

use crate::components::user_interface::weapon_selection::WeaponSelection;

pub fn spawn_weapon_selection_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
        .with_children(|parent| {
            let weapon_icon = WeaponSelection::new_selected_blaster_icon();
            let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
            parent.spawn((weapon_icon_node_bundle(), UiImage::new(texture)));

            let weapon_icon = WeaponSelection::new_unselected_torpedo_icon();
            let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
            parent.spawn((weapon_icon_node_bundle(), UiImage::new(texture)));

            let weapon_icon = WeaponSelection::new_unselected_mine_icon();
            let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
            parent.spawn((weapon_icon_node_bundle(), UiImage::new(texture)));

            let weapon_icon = WeaponSelection::new_unselected_exotic_icon();
            let texture: Handle<Image> = asset_server.load(weapon_icon.icon.to_string());
            parent.spawn((weapon_icon_node_bundle(), UiImage::new(texture)));
        });
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
