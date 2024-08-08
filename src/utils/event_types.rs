use super::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Number;


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    RoomData {payload: RoomDataType},
    Authenticated {payload: SessionIdType},
    Error {payload: ErrorType},
    PlayerJoined {payload: PlayerDataType},
    PlayerLeft {payload: SessionIdType},
    PlayerBanned {payload: PlayerInfoType},
    PlayerUnbanned {payload: PlayerInfoType},
    SettingsChanged {payload: RoomDataType},
    HostChanged {payload: PlayerInfoType},
    GameStarted {},
    RoundStarted {payload: RoundStartType},
    RequestMove {payload: RequestMoveType},
    Action {payload: ActionType},
    PlayerAction {payload: PlayerActionType},
    PlayerDamageReceived {payload: PlayerDamageReceivedType},
    RoundOver {payload: EndType},
    GameOver {payload: EndType},
    GameReset {payload: RoomDataType},
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDataType {
    room_data: RoomData,
}

pub type ErrorType = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDataType {
    player_data: PlayerData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionIdType {
    session_id: SessionId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoType {
    player_info: PlayerInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundStartType {
    starts_at: Number,
    room_data: RoomData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMoveType {
    pub game_state: GameState,
    pub players: Vec<PlayerData>
}

/// Represents a collection of moves to be sent to the server for the
/// current piece
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    pub commands: Vec<Command>
}

impl ActionType {
    pub fn new() -> ActionType {
        ActionType { commands: Vec::new() }
    }

    /// Adds a new command to the end of the vector
    /// containing the requested Commands
    pub fn push(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }

    /// Appends all of the Commands in commands to the 
    /// end of the vector containing the requested Commands
    pub fn append(&mut self, commands: &mut Vec<Command>) -> &mut Self {
        self.commands.append(commands);
        self
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerActionType {
    session_id: SessionId,
    commands: Vec<Command>,
    game_state: GameState,
    events: Vec<GameEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDamageReceivedType {
    session_id: SessionId,
    damage: Number,
    game_state: GameState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndType {
    winner_id: SessionId,
    winner_info: PlayerInfo,
    room_data: RoomData,
}
