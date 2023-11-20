use bevy::{ecs::system::Resource, math::Vec2};

#[derive(Resource)]
pub struct Chunks {
    pub chunks: Vec<Chunk>,
}

pub struct Chunk {
    //simple chunk location e.g. 5,2
    pub chunk_id: Vec2,
    pub chunk_location: Vec2,
}

impl Chunks {
    pub fn chunk_exists(&self, chunk_id: Vec2) -> bool {
        for chunk in &self.chunks {
            if chunk.chunk_id == chunk_id {
                return true;
            }
        }
        false
    }
}
