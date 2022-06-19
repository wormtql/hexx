use crate::common::board::HexBoard;
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::prior::prior::Prior;

pub struct PatternPrior;

impl Prior for PatternPrior {
    fn prior(&self, board: &HexBoard, last_move: Option<(usize, usize)>, next_player: Player, out: &mut [f64]) {
        let size = board.size;
        // if board.empty_count > size * size - 10 {
        //     return;
        // }
        let mut two_dis = board.two_distance_sum(next_player);

        let max = *two_dis.iter().max().unwrap();
        for i in 0..size * size {
            two_dis[i] = max - two_dis[i];
        }

        let mut score = [0.0; MAX_SIZE * MAX_SIZE];
        board.pattern_score(&mut score, last_move, next_player);

        let mut sum = 0.0;
        for i in 0..size * size {
            sum += two_dis[i] as f64 * score[i];
        }

        for i in 0..size * size {
            out[i] = two_dis[i] as f64 * score[i] / sum;
        }

        // board.pattern_score(&mut out[..], last_move, next_player);
    }
}