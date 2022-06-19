use crate::common::board::HexBoard;
use crate::common::player::Player;

pub trait Prior {
    fn prior(&self, board: &HexBoard, last_move: Option<(usize, usize)>, next_player: Player, out: &mut [f64]);
}