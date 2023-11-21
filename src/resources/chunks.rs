use bevy::{
    ecs::{entity::Entity, system::Resource},
    math::Vec2,
};

#[derive(Resource)]
pub struct Chunks {
    pub chunks: Vec<Chunk>,
}

pub struct Chunk {
    //simple chunk location e.g. 5,2
    pub chunk_id: Vec2,
    pub chunk_location: Vec2,
    pub chunk_visible: bool,
    pub space_entity: Entity,
    pub chunk_background: String,
}

impl Chunks {
    pub fn find_chunk(&mut self, chunk_id: Vec2) -> Result<&mut Chunk, &str> {
        let chunks = &mut self.chunks;
        for chunk in chunks {
            if chunk.chunk_id == chunk_id {
                return Ok(chunk);
            }
        }
        return Err("Failed to find chunk");
    }

    pub fn chunk_exists(&mut self, chunk_id: Vec2) -> bool {
        if self.find_chunk(chunk_id).is_ok() {
            return true;
        }
        return false;
    }

    pub fn set_chunk_visibility(&mut self, chunk_id: Vec2, visibility: bool) {
        let chunk = self.find_chunk(chunk_id);
        if chunk.is_ok() {
            chunk.unwrap().chunk_visible = visibility;
        }
    }

    pub fn set_space_entity(&mut self, chunk_id: Vec2, space_entity: Entity) {
        let chunk = self.find_chunk(chunk_id);
        if chunk.is_ok() {
            chunk.unwrap().space_entity = space_entity;
        }
    }

    pub fn get_visibility(&mut self, chunk_id: Vec2) -> bool {
        let chunk = self.find_chunk(chunk_id);
        if chunk.is_ok() {
            return chunk.unwrap().chunk_visible;
        } else {
            return false;
        }
    }
}
