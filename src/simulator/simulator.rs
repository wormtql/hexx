use crate::common::board::HexBoard;
use crate::common::player::Player;

pub trait Simulator {
    /// simulate once and get the winner
    fn simulate_once(&self, board: &HexBoard, next_player: Player) -> Player;

    /// return how many times RED wins
    fn simulate(&self, board: &HexBoard, next_player: Player, count: usize) -> usize {
        let mut result = 0;
        for _ in 0..count {
            if Player::Red == self.simulate_once(board, next_player) {
                result += 1;
            }
        }
        result
    }
}