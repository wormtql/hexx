use crate::common::board::{CellState, HexBoard};
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::cutoff::cutoff::Cutoff;
use crate::inferior_cell::inferior_cell::{get_inferior1, get_inferior2, InferiorCellType};

pub struct InferiorCellCutoff;

impl Cutoff for InferiorCellCutoff {
    fn cutoff(&self, board: &HexBoard, next_player: Player, _last_move: Option<(usize, usize)>, out: &mut [bool]) {
        let size = board.size;
        let mut board = board.clone();

        let mut flag = false;
        while flag {
            flag = false;

            let mut temp = [(0, 0); MAX_SIZE * MAX_SIZE];
            let mut out_count = 0;
            flag = flag || get_inferior1(&board, InferiorCellType::Dead1, |x, y| {
                temp[out_count] = (x, y);
                out_count += 1;
            });
            for i in 0..out_count {
                let (x, y) = temp[i];
                board.set(x, y, CellState::Red);
                out[x * size + y] = true;
            }

            let mut temp = [(0, 0); MAX_SIZE * MAX_SIZE];
            let mut out_count = 0;
            flag = flag || get_inferior2(&board, next_player, InferiorCellType::Captured2, |x, y| {
                temp[out_count] = (x, y);
                out_count += 1;
            });
            for i in 0..out_count {
                let (x, y) = temp[i];
                board.set(x, y, next_player.to_cell());
                out[x * size + y] = true;
            }
        }
        // get_inferior2(&board, next_player, InferiorCellType::Inferior2, |x, y| {
        //     out[x * size + y] = true;
        // });
        // get_inferior1(&board, InferiorCellType::Inferior1, |x, y| {
        //     out[x * size + y] = true;
        // });

    }
}
