use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SpaceZoneBorder {
    pub top_border: f32,
    pub left_border: f32,
    pub bottom_border: f32,
    pub right_border: f32,
}
