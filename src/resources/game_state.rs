use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GameState {
    pub paused: bool,
}
