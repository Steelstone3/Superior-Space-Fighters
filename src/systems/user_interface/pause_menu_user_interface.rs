use bevy::{
    ecs::system::{Commands, Query},
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::default,
    render::color::Color,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        AlignItems, BorderColor, Display, GridTrack, JustifyContent, JustifyItems, Style, UiRect,
        Val,
    },
};

use crate::{
    components::user_interface::{
        main_menu_buttons::{LoadGameButton, QuitGameButton, SaveGameButton},
        pause_menu_parent::PauseMenuParent,
    },
    queries::pause_menu_parent_queries::PauseMenuParentEntityQuery,
};

pub fn spawn_pause_menu_user_interface(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(PauseMenuParent)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Superior Space Fighters",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::ANTIQUE_WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(SaveGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Save Game",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(LoadGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Load Game",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(QuitGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit Game",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
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
