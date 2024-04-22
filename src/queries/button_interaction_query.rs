use bevy::{
    ecs::query::{Changed, QueryData, QueryFilter, With},
    ui::{widget::Button, BackgroundColor, BorderColor, Interaction},
};

use crate::components::user_interface::main_menu_buttons::{
    LoadGameButton, NewGameButton, SaveGameButton,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ButtonInteractionMutableQuery {
    pub interaction: &'static Interaction,
    pub background_colour: &'static mut BackgroundColor,
    pub border_colour: &'static mut BorderColor,
    pub new_game_button: Option<&'static NewGameButton>,
    pub save_game_button: Option<&'static SaveGameButton>,
    pub load_game_button: Option<&'static LoadGameButton>,
}

#[derive(QueryFilter)]
pub struct ButtonInteractionFilter {
    pub with_changed_interaction: Changed<Interaction>,
    pub with_button: With<Button>,
}
