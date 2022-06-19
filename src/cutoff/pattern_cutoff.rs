use crate::common::board::HexBoard;
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::cutoff::cutoff::Cutoff;

pub struct PatternCutoff;

impl Cutoff for PatternCutoff {
    fn cutoff(&self, board: &HexBoard, next_player: Player, last_move: Option<(usize, usize)>, out: &mut [bool]) {
        let mut score = [0.0; MAX_SIZE * MAX_SIZE];

        let consider = board.pattern_score(&mut score[..], last_move, next_player);

        let ss = board.size * board.size;
        for i in 0..ss {
            if !consider[i] {
                out[i] = true;
            }
        }
    }
}