use crate::common::board::CellState;
use serde::{Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize)]
pub enum Player {
    Red,
    Blue
}

impl Player {
    pub fn reverse(self) -> Player {
        if self == Player::Red {
            Player::Blue
        } else {
            Player::Red
        }
    }

    pub fn to_cell(self) -> CellState {
        if self == Player::Red {
            CellState::Red
        } else {
            CellState::Blue
        }
    }
}
