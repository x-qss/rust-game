pub trait Gamemode {
    fn update(&mut self);
    fn on_player_join(&mut self, player_uuid: i32);
}
