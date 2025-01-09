use servify;

#[derive(Debug, Clone, Copy)]
pub struct InstanceID(u32);

#[derive(Debug, Clone, Copy)]
pub struct PlayerID(u32);

#[derive(Debug)]
pub enum InstanceMessage {
    JoinInstance(PlayerID),
}

#[servify::service(
    impls = [
        Instance_join,
    ]
)]
#[derive(Debug)]
pub struct Instance {
    instance_id: InstanceID,
    players: Vec<PlayerID>,
}

#[servify::export]
impl Instance {
    fn join(&mut self, player_id: PlayerID) -> Vec<PlayerID> {
        self.players.push(player_id);
        self.players.clone()
    }
}
