use std::collections::HashMap;
use itertools::Itertools;
use smallvec::SmallVec;
use rand::Rng;
use lazy_static::lazy_static;
use crate::common::board::{BoardPosition, CellState, HexBoard};
use crate::common::constants::{DIS_PATTERN_12, DIS_PATTERN_6, MAX_SIZE};
use crate::common::player::Player;

#[derive(Debug, Clone)]
pub struct PatternItem {
    pub pattern_type: usize,
    pub gamma: f64,
    pub key: u64,
    pub killer: usize,
    pub pattern: [usize; 12],
    pub size: usize,
    pub wins: usize,
    pub total: usize,
}

impl PatternItem {
    pub fn flip_color_and_mirror(&self, zobrist_keys: &ZobristKeys) -> PatternItem {
        let mut pattern = self.pattern.clone();

        let mirror = [
            2,  4,  0,  5,  1,  3,
            10, 11,  8, 9,  6,  7,
        ];

        for i in 0..self.size {
            pattern[i] = self.pattern[mirror[i]];
        }

        let color_map = [0, 2, 1, 4, 3, 5];
        for i in 0..self.size {
            pattern[i] = color_map[pattern[i]];
        }
        let key = zobrist_keys.generate_pattern_key(&pattern[..self.size]);

        let mut result = self.clone();

        result.key = key;
        if result.pattern_type == 3 {
            result.killer = mirror[self.killer];
        }
        result.pattern = pattern;

        result
    }

    pub fn rotate_180(&self, zobrist_keys: &ZobristKeys) -> PatternItem {
        let mut result = self.clone();

        let mut pattern = self.pattern.clone();
        let map = [5, 4, 3, 2, 1, 0, 11, 10, 9, 8, 7, 6];
        for i in 0..self.size {
            pattern[i] = self.pattern[map[i]];
        }

        let key = zobrist_keys.generate_pattern_key(&pattern[..self.size]);

        result.key = key;
        if result.pattern_type == 3 {
            result.killer = map[self.killer];
        }
        result.pattern = pattern;

        result
    }
}

pub struct ZobristKeys {
    cell_keys: [[u64; 6]; 12],
    size_keys: [u64; 2],
}

impl ZobristKeys {
    pub fn new() -> Self {
        let mut result = ZobristKeys {
            cell_keys: [[0; 6]; 12],
            size_keys: [0; 2]
        };

        for i in 0..12 {
            for j in 0..6 {
                let r = rand::thread_rng().gen::<u64>();
                result.cell_keys[i][j] = r;
            }
        }

        for i in 0..2 {
            let r = rand::thread_rng().gen::<u64>();
            result.size_keys[i] = r;
        }

        result
    }

    pub fn generate_pattern_key(&self, pattern: &[usize]) -> u64 {
        let mut key = if pattern.len() == 6 { self.size_keys[0] } else { self.size_keys[1] };

        for (index, &cell) in pattern.iter().enumerate() {
            let cell = if cell == 5 { 3 } else { cell };
            key ^= self.cell_keys[index][cell];
        }

        key
    }

    pub fn get_pattern_key_from_board(&self, board: &HexBoard, x: usize, y: usize) -> (u64, u64) {
        let mut key6 = self.size_keys[0];
        let mut key12 = 0;

        for i in 0..6 {
            let (nx, ny) = (x as i32 + DIS_PATTERN_12[i][0], y as i32 + DIS_PATTERN_12[i][1]);
            let position = board.get_position(nx, ny);
            if position == BoardPosition::RightTop || position == BoardPosition::LeftBottom {
                key6 ^= self.cell_keys[i][4];
            } else if position == BoardPosition::RightBottom || position == BoardPosition::LeftTop {
                key6 ^= self.cell_keys[i][3];
            } else if let BoardPosition::Cell(x) = position {
                let index = if x == CellState::Red {
                    1
                } else if x == CellState::Blue {
                    2
                } else {
                    0
                };
                key6 ^= self.cell_keys[i][index];
            }
        }

        key12 = self.size_keys[0] ^ self.size_keys[1] ^ key6;
        for i in 6..12 {
            let (nx, ny) = (x as i32 + DIS_PATTERN_12[i][0], y as i32 + DIS_PATTERN_12[i][1]);
            let position = board.get_position(nx, ny);
            if position == BoardPosition::RightTop || position == BoardPosition::LeftBottom {
                key12 ^= self.cell_keys[i][4];
            } else if position == BoardPosition::RightBottom || position == BoardPosition::LeftTop {
                key12 ^= self.cell_keys[i][3];
            } else if let BoardPosition::Cell(x) = position {
                let index = if x == CellState::Red {
                    1
                } else if x == CellState::Blue {
                    2
                } else {
                    0
                };
                key12 ^= self.cell_keys[i][index];
            }
        }

        (key6, key12)
    }
}

pub struct Patterns {
    pub zobrist_keys: ZobristKeys,

    pub patterns_red: HashMap<u64, PatternItem>,
    pub patterns_blue: HashMap<u64, PatternItem>,
}

impl Patterns {
    fn insert_hashtable(&mut self, key: u64, pattern: PatternItem, player: Player) -> bool {
        let table = if player == Player::Red {
            &mut self.patterns_red
        } else {
            &mut self.patterns_blue
        };
        if !table.contains_key(&key) {
            table.insert(key, pattern);
            true
        } else {
            let item = table.get(&key).unwrap();
            if item.gamma == pattern.gamma {
                return false;
            } else if (item.pattern_type > 0) ^ (pattern.pattern_type > 0) {
                panic!("Prunable classification mismatch.");
            } else if item.pattern_type == 0 && pattern.gamma > item.gamma {
                table.insert(key, pattern);
            }
            false
        }
    }

    pub fn get_pattern_item(&self, key: &u64, player: Player) -> Option<&PatternItem> {
        let table = if player == Player::Red { &self.patterns_red } else { &self.patterns_blue };
        table.get(key)
    }

    pub fn load_patterns(&mut self, global_data: &str) {
        for line in global_data.lines() {
            let temp = line.split_whitespace().collect_vec();

            let gamma = temp[0].parse::<f64>().unwrap();
            let wins = temp[1].parse::<usize>().unwrap();
            let total = temp[2].parse::<usize>().unwrap();
            let pattern = temp[3];
            let t = temp[4].parse::<usize>().unwrap();
            let killer = temp[5].parse::<usize>().unwrap();

            let size = pattern.len();
            let mut p = [0_usize; 12];
            for (index, c) in pattern.chars().enumerate() {
                let x = c as usize - '0' as usize;
                p[index] = x;
            }

            let pattern_key = self.zobrist_keys.generate_pattern_key(&p[..size]);
            let pattern_item = PatternItem {
                pattern_type: t,
                gamma,
                key: pattern_key,
                killer: if killer > 0 { killer - 1 } else { 0 },
                pattern: p,
                size,
                wins,
                total
            };

            let mut rotated_pattern = pattern_item.rotate_180(&self.zobrist_keys);

            let blue_item = pattern_item.flip_color_and_mirror(&self.zobrist_keys);
            let blue_item_rot = blue_item.rotate_180(&self.zobrist_keys);

            self.insert_hashtable(pattern_key, pattern_item, Player::Red);
            self.insert_hashtable(rotated_pattern.key, rotated_pattern, Player::Red);

            self.insert_hashtable(blue_item.key, blue_item, Player::Blue);
            self.insert_hashtable(blue_item_rot.key, blue_item_rot, Player::Blue);
        }
    }

    pub fn new() -> Patterns {
        Patterns {
            zobrist_keys: ZobristKeys::new(),
            patterns_red: HashMap::new(),
            patterns_blue: HashMap::new(),
        }
    }

    pub fn from_data(data: &str) -> Self {
        let mut result = Patterns::new();
        result.load_patterns(&data);
        result
    }
}

lazy_static! {
    pub static ref GLOBAL_PATTERNS: Patterns = {
        let data = include_str!("../../data/mohex-global-pattern-gamma.txt");
        Patterns::from_data(&data)
    };

    pub static ref LOCAL_PATTERNS: Patterns = {
        let data = include_str!("../../data/mohex-local-pattern-gamma.txt");
        Patterns::from_data(&data)
    };
}

fn get_pattern_both(patterns: &Patterns, key6: u64, key12: u64, player: Player) -> Option<&PatternItem> {
    if let Some(x) = patterns.get_pattern_item(&key12, player) {
        return Some(x);
    }
    if let Some(x) = patterns.get_pattern_item(&key6, player) {
        return Some(x);
    }
    None
}

fn get_pattern_move(x: i32, y: i32, index: usize) -> (i32, i32) {
    let (nx, ny) = (x + DIS_PATTERN_12[index][0], y + DIS_PATTERN_12[index][1]);
    (nx, ny)
}

impl HexBoard {
    pub fn pattern_score(&self, out: &mut[f64], last_move: Option<(usize, usize)>, next_player: Player) -> [bool; MAX_SIZE * MAX_SIZE] {
        let size = self.size;

        let mut total_score = 0.0;
        let mut temp = [0.0; MAX_SIZE * MAX_SIZE];

        let mut safe = [false; MAX_SIZE * MAX_SIZE];
        let mut pruned = [false; MAX_SIZE * MAX_SIZE];
        let mut consider = [false; MAX_SIZE * MAX_SIZE];

        for x in 0..size {
            for y in 0..size {
                if self.is_empty(x, y) {
                    let (key6, key12) = GLOBAL_PATTERNS.zobrist_keys.get_pattern_key_from_board(&self, x, y);
                    let pos = x * size + y;

                    let pattern = get_pattern_both(&GLOBAL_PATTERNS, key6, key12, next_player);
                    if let Some(p) = pattern {
                        if p.pattern_type == 1 || p.pattern_type == 2 {
                            pruned[pos] = true;
                        } else if p.pattern_type == 3 {
                            let (nx, ny) = get_pattern_move(x as i32, y as i32, p.killer);
                            if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);
                                let npos = nx * size + ny;

                                if !pruned[npos] {
                                    safe[npos] = true;
                                    pruned[pos] = true;
                                }
                            } else {
                                panic!("killer not empty, ({}, {}), {:?}", x, y, p);
                            }
                        } else {
                            total_score += p.gamma;
                            temp[pos] = p.gamma;
                            consider[pos] = true;
                        }
                    } else {
                        total_score += 1.0;
                        temp[pos] = 1.0;
                        consider[pos] = true;
                    }
                }
            }
        }

        if let Some((x, y)) = last_move {
            for i in 0..12 {
                let (nx, ny) = (x as i32 + DIS_PATTERN_12[i][0], y as i32 + DIS_PATTERN_12[i][1]);
                if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let pos = nx * size + ny;
                    if self.is_empty(nx, ny) && consider[pos] {
                        let (key6, key12) = LOCAL_PATTERNS.zobrist_keys.get_pattern_key_from_board(&self, nx, ny);

                        let pattern = get_pattern_both(&LOCAL_PATTERNS, key6, key12, next_player);
                        if let Some(x) = pattern {
                            total_score += x.gamma;
                            temp[pos] += x.gamma;
                        }
                    }
                }
            }
        }

        for i in 0..size * size {
            out[i] = temp[i] / total_score;
        }

        consider
    }
}

#[cfg(test)]
mod test {
    use crate::common::board::{CellState, HexBoard};
    use crate::common::constants::MAX_SIZE;
    use crate::common::player::Player;
    use crate::pattern::pattern::GLOBAL_PATTERNS;

    // #[test]
    // fn test_pattern_1() {
    //     let p = [0, 2, 1, 2, 2, 1, 2, 1, 0, 1, 0, 2];
    //     let key = GLOBAL_PATTERNS.generate_pattern_key(&p);
    //     println!("key: {}", key);
    //
    //     let item = GLOBAL_PATTERNS.patterns.get(&key).unwrap();
    //     println!("{:?}", item);
    //
    //     assert!(false);
    // }

    // #[test]
    // fn test_score_1() {
    //     let mut board = HexBoard::new(5);
    //     let mut scores = vec![0.0; board.size * board.size];
    //
    //     board.pattern_score(&mut scores, Some((2, 2)), Player::Red);
    //     println!("{:?}", scores);
    //     // let sum: f64 = scores.iter().sum();
    //     // println!("{}", sum);
    //
    //     assert!(false);
    // }

    #[test]
    fn test_pattern_2() {
        let mut board = HexBoard::new(11);
        board.set(2, 10, CellState::Red);
        board.set(3, 9, CellState::Blue);
        board.set(4, 9, CellState::Red);

        let key = GLOBAL_PATTERNS.zobrist_keys.get_pattern_key_from_board(&board, 3, 10).0;

        let p = [1, 4, 2, 4, 1, 0];
        let key2 = GLOBAL_PATTERNS.zobrist_keys.generate_pattern_key(&p);

        assert_eq!(key, key2);
    }

    #[test]
    fn test_pattern_3() {
        let mut board = HexBoard::new(11);

        let key = GLOBAL_PATTERNS.zobrist_keys.get_pattern_key_from_board(&board, 5, 5).1;

        let p = [0; 12];
        let key2 = GLOBAL_PATTERNS.zobrist_keys.generate_pattern_key(&p);

        assert_eq!(key, key2);
    }

    #[test]
    fn test_pattern_4() {
        let mut board = HexBoard::new(11);

        let key = GLOBAL_PATTERNS.zobrist_keys.get_pattern_key_from_board(&board, 5, 5).0;

        let p = [0; 6];
        let key2 = GLOBAL_PATTERNS.zobrist_keys.generate_pattern_key(&p);

        assert_eq!(key, key2);
    }

    #[test]
    fn test_score_2() {
        let mut board = HexBoard::new(5);
        board.set(2, 2, CellState::Red);

        let mut scores = [0.0; 25];
        board.pattern_score(&mut scores, Some((2, 2)), Player::Blue);
        println!("{:?}", scores);

        assert!(false);
    }
}