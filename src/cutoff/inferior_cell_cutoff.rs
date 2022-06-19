use crate::common::board::HexBoard;
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::cutoff::cutoff::Cutoff;
use crate::inferior_cell::inferior_cell::{get_inferior1, get_inferior2, InferiorCellType};

pub struct InferiorCellCutoff;

impl Cutoff for InferiorCellCutoff {
    fn cutoff(&self, board: &HexBoard, next_player: Player, _last_move: Option<(usize, usize)>, out: &mut [bool]) {
        let size = board.size;
        get_inferior2(&board, next_player, InferiorCellType::Inferior2, |x, y| {
            out[x * size + y] = true;
        });
        // get_inferior1(&board, InferiorCellType::Inferior1, |x, y| {
        //     out[x * size + y] = true;
        // });

    }
}
