use crate::common::board::HexBoard;
use crate::common::player::Player;

pub trait Cutoff {
    fn cutoff(&self, board: &HexBoard, next_player: Player, last_move: Option<(usize, usize)>, out: &mut [bool]);
}