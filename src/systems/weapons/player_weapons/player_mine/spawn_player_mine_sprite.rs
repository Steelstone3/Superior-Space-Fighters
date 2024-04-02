use crate::{
    components::weapons::player_weapons::player_mine::PlayerMine,
    events::{game_events::FirePlayerMineEvent, spawn_sprite_event::SpawnSpriteEvent},
    queries::player_starship_queries::PlayerStarshipTransformQuery,
};
use bevy::{
    ecs::event::{EventReader, EventWriter},
    prelude::{Commands, Query},
};

pub fn spawn_player_mine_sprite(
    mut commands: Commands,
    mut fire_player_mine_event: EventReader<FirePlayerMineEvent>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    player_starships: Query<PlayerStarshipTransformQuery>,
) {
    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    for _ in fire_player_mine_event.read() {
        let mut player_transform = *player_starship.transform;
        let mine_size = 100.0;
        let mine_spawn_position =
            player_transform.translation + player_transform.down() * (mine_size / 1.75);
        player_transform.translation = mine_spawn_position;
        player_transform.translation.z = 3.0;
        let mine = PlayerMine::default();

        let image_path = mine.mine.mine.to_string();
        let size = mine.mine.lifetime_weapon.weapon.size;
        let entity = commands.spawn(mine).id();

        let event = SpawnSpriteEvent {
            sprite_path: image_path,
            size,
            transform: player_transform,
            entity,
        };

        spawn_sprite_event.send(event);
    }
}
