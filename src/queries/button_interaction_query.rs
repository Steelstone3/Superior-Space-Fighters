use bevy::{
    ecs::query::{Changed, QueryData, QueryFilter, With},
    ui::{widget::Button, BackgroundColor, BorderColor, Interaction},
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ButtonInteractionMutableQuery {
    pub interaction: &'static Interaction,
    pub background_colour: &'static mut BackgroundColor,
    pub border_colour: &'static mut BorderColor,
}

#[derive(QueryFilter)]
pub struct ButtonInteractionFilter {
    pub with_changed_interaction: Changed<Interaction>,
    pub with_button: With<Button>,
}
