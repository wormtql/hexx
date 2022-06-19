use crate::common::board::HexBoard;
use crate::common::player::Player;

pub trait Solver {
    fn solve(&self, board: &HexBoard, next_player: Player) -> (usize, usize);
}