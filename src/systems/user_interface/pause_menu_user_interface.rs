use bevy::{
    ecs::system::{Commands, Query},
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::default,
    render::color::Color,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Display, GridTrack, PositionType, Style, Val,
    },
};

use crate::{
    components::user_interface::pause_menu_parent::PauseMenuParent,
    queries::pause_menu_parent_queries::PauseMenuParentEntityQuery,
};

pub fn spawn_pause_menu_user_interface(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                //Must set specific width + height otherwise images wont know what size to display
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
        .insert(PauseMenuParent {})
        .with_children(|parent| {
            parent.spawn(TextBundle::from("Paused").with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                top: Val::Px(10.0),
                ..default()
            }));
        });
}

pub fn despawn_pause_menu_user_interface(
    mut commands: Commands,
    menu_parent_query: Query<PauseMenuParentEntityQuery>,
) {
    if let Ok(menu_parent) = menu_parent_query.get_single() {
        commands.entity(menu_parent.entity).despawn_recursive();
    }
}
