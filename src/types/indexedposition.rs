use crate::types::position::Position;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IndexedPosition {
    pub position: Position,
    pub index: i32,
}
