use crate::common::board::{CellState, HexBoard};
use lazy_static::lazy_static;
use crate::common::constants::{DIS1, DIS2_DIAG, DIS2_H, DIS2_V, MAX_SIZE};
use crate::common::player::Player;
use crate::inferior_cell::pattern::PatternDict;

lazy_static! {
    static ref PATTERNS_DICT: PatternDict = PatternDict::new();
}

impl HexBoard {
    pub fn get_encoding1(&self, x: usize, y: usize) -> usize {
        let mut result = 0;

        for i in 0..6 {
            let nx = x as i32 + DIS1[i][0];
            let ny = y as i32 + DIS1[i][1];

            let cell = self.get_with_padding(nx, ny);
            if cell == CellState::Red {
                result = (result << 2) | 1;
            } else if cell == CellState::Blue {
                result = (result << 2) | 2;
            } else {
                result <<= 2;
            }
        }

        result
    }

    pub fn get_encoding2(&self, x: usize, y: usize, direction: char) -> usize {
        let mut result = 0;

        let dis = if direction == 'v' { &DIS2_V } else if direction == 'h' { &DIS2_H } else if direction == 'd' { &DIS2_DIAG } else {
            panic!("direction not found");
        };
        for i in 0..8 {
            let nx = x as i32 + dis[i][0];
            let ny = y as i32 + dis[i][1];

            let cell = self.get_with_padding(nx, ny);
            if cell == CellState::Red {
                result = (result << 2) | 1;
            } else if cell == CellState::Blue {
                result = (result << 2) | 2;
            } else {
                result <<= 2;
            }
        }

        result
    }

    pub fn is_vacant2(&self, x: usize, y: usize, direction: char) -> bool {
        let current = self.get(x, y);
        current == CellState::Empty && {
            let (dx, dy) = if direction == 'v' {
                (1, 0)
            } else if direction == 'h' {
                (0, 1)
            } else if direction == 'd' {
                (1, -1)
            } else {
                panic!("direction error")
            };
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let cell = self.get_with_padding(nx, ny);
            cell == CellState::Empty
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InferiorCellType {
    Dead1,
    Inferior1,
    Captured2,
    Inferior2,
}

/// return: whether there is a inferior cell
pub fn get_inferior1<T: FnMut(usize, usize) -> ()>(board: &HexBoard, t: InferiorCellType, mut action: T) -> bool {
    let p = match t {
        InferiorCellType::Dead1 => &PATTERNS_DICT.dead1,
        InferiorCellType::Inferior1 => &PATTERNS_DICT.inferior1,
        _ => unreachable!()
    };
    let size = board.size;

    let mut temp = [[false; MAX_SIZE]; MAX_SIZE];
    let mut flag = false;
    for x in 0..board.size {
        for y in 0..board.size {
            if board.is_empty(x, y) {
                let encoding = board.get_encoding1(x, y);
                let (test_result, critical_index) = p.patterns_map[encoding];
                if test_result {
                    if critical_index < usize::MAX {
                        assert_eq!(t, InferiorCellType::Inferior1);
                        let cx = x as i32 + DIS1[critical_index][0];
                        let cy = y as i32 + DIS1[critical_index][1];
                        if cx >= 0 && cx < size as i32 && cy >= 0 && cy < size as i32 {
                            let (cx, cy) = (cx as usize, cy as usize);
                            flag = true;
                            temp[x][y] = true;
                            temp[cx][cy] = false;
                        }
                    } else {
                        flag = true;
                        temp[x][y] = true;
                    }
                }
            }
        }
    }

    for x in 0..size {
        for y in 0..size {
            if temp[x][y] {
                action(x, y);
            }
        }
    }

    flag
}

pub fn get_inferior2<T: FnMut(usize, usize) -> ()>(board: &HexBoard, player: Player, t: InferiorCellType, mut action: T) -> bool {
    let p = match t {
        InferiorCellType::Captured2 => {
            if player == Player::Red { &PATTERNS_DICT.captured_red2 } else { &PATTERNS_DICT.captured_blue2 }
        },
        InferiorCellType::Inferior2 => {
            if player == Player::Red { &PATTERNS_DICT.inferior_red2 } else { &PATTERNS_DICT.inferior_blue2 }
        },
        _ => unreachable!()
    };
    let size = board.size as i32;

    let mut temp = [[false; MAX_SIZE]; MAX_SIZE];
    // let mut vis = [[false; MAX_SIZE]; MAX_SIZE];
    let mut critical = [[false; MAX_SIZE]; MAX_SIZE];
    let mut flag = false;
    for x in 0..board.size {
        for y in 0..board.size {
            if critical[x][y] {
                continue;
            }

            for d in ['v', 'h', 'd'] {
                let (other_x, other_y) = if d == 'v' {
                    (x as i32 + 1, y as i32)
                } else if d == 'h' {
                    (x as i32, y as i32 + 1)
                } else {
                    (x as i32 + 1, y as i32 - 1)
                };

                // if other_x >= 0 && other_x < size && other_y >= 0 && other_y < size && board.is_vacant2(x, y, d) {
                if other_x >= 0 && other_x < size && other_y >= 0 && other_y < size && board.is_vacant2(x, y, d) && !critical[other_x as usize][other_y as usize] {
                    let encoding = board.get_encoding2(x, y, d);
                    let (test_result, critical_index) = p.patterns_map[encoding];
                    if test_result {
                        if critical_index < usize::MAX {
                            assert_eq!(t, InferiorCellType::Inferior2);
                            let dis = if d == 'v' { &DIS2_V } else if d == 'h' { &DIS2_H } else { &DIS2_DIAG };
                            let cx = x as i32 + dis[critical_index][0];
                            let cy = y as i32 + dis[critical_index][1];
                            if cx >= 0 && cx < size && cy >= 0 && cy < size {
                                let (cx, cy) = (cx as usize, cy as usize);

                                if !temp[cx][cy] {
                                    critical[cx][cy] = true;
                                    temp[x][y] = true;
                                    temp[other_x as usize][other_y as usize] = true;
                                    flag = true;
                                }
                                // temp[cx][cy] = false;
                                // temp[x][y] = true;
                                // temp[other_x as usize][other_y as usize] = true;
                                // flag = true;
                                // critical[cx as usize][cy as usize] = true;
                                // if !vis[x][y] {
                                //     vis[x][y] = true;
                                //     action(x, y);
                                // }
                                // if !vis[other_x as usize][other_y as usize] {
                                //     vis[other_x as usize][other_y as usize] = true;
                                //     action(other_x as usize, other_y as usize);
                                // }
                                // flag = true;
                            }
                        } else {
                            assert_eq!(t, InferiorCellType::Captured2);
                            temp[x][y] = true;
                            temp[other_x as usize][other_y as usize] = true;
                            flag = true;
                        };
                    }
                }
            }


        }
    }

    for x in 0..board.size {
        for y in 0..board.size {
            if temp[x][y] {
                action(x, y);
            }
        }
    }

    flag
}

pub fn fill_dead_and_captured(board: &HexBoard, next_player: Player) -> HexBoard {
    let mut flag = true;
    let mut result = board.clone();

    while flag {
        flag = false;

        let mut out = [(0, 0); MAX_SIZE * MAX_SIZE];
        let mut out_count = 0;
        flag = flag || get_inferior1(&result, InferiorCellType::Dead1, |x, y| {
            out[out_count] = (x, y);
            out_count += 1;
        });
        for i in 0..out_count {
            let (x, y) = out[i];
            result.set(x, y, CellState::Red);
        }

        let mut out = [(0, 0); MAX_SIZE * MAX_SIZE];
        let mut out_count = 0;
        flag = flag || get_inferior2(&result, next_player, InferiorCellType::Captured2, |x, y| {
            out[out_count] = (x, y);
            out_count += 1;
        });
        for i in 0..out_count {
            let (x, y) = out[i];
            result.set(x, y, next_player.to_cell());
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::common::board::{CellState, HexBoard};
    use crate::common::player::Player;
    use crate::inferior_cell::inferior_cell::{fill_dead_and_captured, get_inferior1, get_inferior2, InferiorCellType};

    #[test]
    pub fn test_inferior1() {
        let mut board = HexBoard::new(3);
        // board.set(0, 1, CellState::Red);
        board.set(0, 2, CellState::Red);
        board.set(1, 0, CellState::Red);
        board.set(2, 1, CellState::Blue);

        let mut temp = [false; 9];
        get_inferior1(&board, InferiorCellType::Inferior1, |x, y| {
            temp[x * 3 + y] = true;
        });
        assert!(temp[4]);
    }

    #[test]
    pub fn test_inferior2() {
        let mut board = HexBoard::new(3);
        // board.set(0, 1, CellState::Red);
        board.set(0, 2, CellState::Blue);
        board.set(1, 0, CellState::Red);
        board.set(2, 0, CellState::Blue);
        board.set(2, 1, CellState::Red);

        let mut temp = [false; 9];
        get_inferior1(&board, InferiorCellType::Inferior1, |x, y| {
            temp[x * 3 + y] = true;
        });
        assert!(!temp[4]);
    }

    #[test]
    pub fn test_inferior4() {
        let mut board = HexBoard::new(5);
        // board.set(0, 1, CellState::Red);
        board.fill_row(0, "__bbb");
        board.fill_row(1, "rbrrr");
        board.fill_row(2, "r____");
        board.fill_row(3, "_____");
        board.fill_row(4, "_____");
        println!("{}", board);

        let board_filled = fill_dead_and_captured(&board, Player::Red);
        println!("{}", board_filled);

        let mut temp = [false; 25];
        // get_inferior1(&board_filled, InferiorCellType::Inferior1, |x, y| {
        //     temp[x * 5 + y] = true;
        // });
        get_inferior2(&board_filled, Player::Red, InferiorCellType::Inferior2, |x, y| {
            temp[x * 5 + y] = true;
        });
        println!("{:?}", temp);
        assert!(false);
    }

    // #[test]
    // pub fn test_inferior3() {
    //     let mut board = HexBoard::new(5);
    //     // board.set(0, 1, CellState::Red);
    //     board.fill_row(0, "_brrr");
    //     board.fill_row(1, "_bbbr");
    //     board.fill_row(2, "_brr_");
    //     board.fill_row(3, "b__b_");
    //     board.fill_row(4, "_____");
    //     println!("{}", board);
    //
    //     let board_filled = fill_dead_and_captured(&board, Player::Red);
    //     println!("{}", board_filled);
    //
    //     let mut temp = [false; 25];
    //     // get_inferior1(&board_filled, InferiorCellType::Inferior1, |x, y| {
    //     //     temp[x * 5 + y] = true;
    //     // });
    //     get_inferior2(&board_filled, Player::Red, InferiorCellType::Inferior2, |x, y| {
    //         temp[x * 5 + y] = true;
    //     });
    //     println!("{:?}", temp);
    //     assert!(false);
    // }
}