use crate::common::board::{CellState, HexBoard};
use crate::common::player::Player;
use crate::simulator::simulator::Simulator;
use lazy_static::lazy_static;
use rand::prelude::*;
use rand::thread_rng;
use smallvec::SmallVec;

pub struct SaveBridgeSimulator;

lazy_static! {
    static ref MOVES: Vec<usize> = {
        let mut result = Vec::new();
        for i in 0..19 * 19 {
            result.push(i);
        }
        result
    };
}

fn save_bridge(board: &mut HexBoard, pos: usize, player: Player) -> bool {
    let dis = [[-1, 0], [-1, 1], [0, 1], [1, 0], [1, -1], [0, -1]];

    let size = board.size as i32;
    let (x, y) = (pos as i32 / size, pos as i32 % size);

    let mut temp: SmallVec<[usize; 3]> = SmallVec::new();

    let mut i = 0;
    while i < 6 {
        let mut flag = false;
        for j in [0, 2] {
            let nx = x + dis[(i + j) % 6][0];
            let ny = y + dis[(i + j) % 6][1];

            let cell = board.get_with_padding(nx, ny);
            let cell_player = cell.get_player();

            if cell_player.is_none() {
                flag = true;
                break;
            }

            let cell_player = cell_player.unwrap();
            if cell_player == player {
                flag = true;
                break;
            }
        }

        if !flag {
            let nx = x + dis[(i + 1) % 6][0];
            let ny = y + dis[(i + 1) % 6][1];

            if nx >= 0 && nx < size && ny >= 0 && ny < size {
                let npos = nx * size + ny;

                if board.get_abs(npos as usize) == CellState::Empty {
                    temp.push(npos as usize);
                    i += 1;
                }
            }
        }

        i += 1;
    }

    if temp.len() == 0 {
        false
    } else if temp.len() == 1 {
        let pos = temp[0];
        board.set_abs(pos, player.reverse().to_cell());
        true
    } else {
        let index = thread_rng().gen::<usize>() % temp.len();
        let pos = temp[index];

        board.set_abs(pos, player.reverse().to_cell());
        true
    }
}

impl Simulator for SaveBridgeSimulator {
    fn simulate_once(&self, board: &HexBoard, next_player: Player) -> Player {
        let mut board = board.clone();
        let ss = board.size * board.size;

        let mut moves = MOVES.clone();
        let mut moves = &mut moves[0..ss];

        let mut next_player = next_player;

        for i in 0..ss {
            let valid_index = thread_rng().gen::<usize>() % (ss - i);
            let position = moves[valid_index];

            if board.get_abs(position) == CellState::Empty {
                board.set_abs(position, next_player.to_cell());

                if !save_bridge(&mut board, position, next_player) {
                    next_player = next_player.reverse();
                }
            }

            moves.swap(valid_index, ss - i - 1);
        }

        board.winner_definite()
    }
}
