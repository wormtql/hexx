use itertools::Itertools;
use crate::common::board::HexBoard;
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::cutoff::cutoff::Cutoff;
use crate::two_distance::two_distance::TwoDistanceStart;

pub struct TwoDistanceCutoff {
    pub rank: usize
}

impl Cutoff for TwoDistanceCutoff {
    fn cutoff(&self, board: &HexBoard, next_player: Player, _last_move: Option<(usize, usize)>, out: &mut [bool]) {
        let mut dis1 = [0; MAX_SIZE * MAX_SIZE];
        let mut dis2 = [0; MAX_SIZE * MAX_SIZE];
        let size = board.size;
        let ss = size * size;

        if next_player == Player::Red {
            board.two_distance(TwoDistanceStart::LeftTop, &mut dis1[..]);
            board.two_distance(TwoDistanceStart::RightBottom, &mut dis2[..]);
        } else {
            board.two_distance(TwoDistanceStart::LeftBottom, &mut dis1[..]);
            board.two_distance(TwoDistanceStart::RightTop, &mut dis2[..]);
        }

        let mut dis = dis1.clone();
        for i in 0..ss {
            dis[i] += dis2[i];
            dis1[i] += dis2[i];
        }

        let mut x = dis.iter().cloned().unique().collect_vec();
        let k = self.rank.min(x.len() - 1);
        let kth = *order_stat::kth(&mut x, k);

        for i in 0..ss {
            if dis1[i] > kth {
                out[i] = true;
            }
        }
    }
}