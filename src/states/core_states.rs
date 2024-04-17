use bevy::ecs::schedule::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Paused,
    #[default]
    MainMenu,
    InGame,
}
