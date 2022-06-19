use std::fmt::{Display, Formatter};
use ansi_term::Colour::{Blue, Red};
use crate::common::constants::{DIS1, MAX_SIZE};
use crate::common::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CellState {
    Empty,
    Red,
    Blue
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BoardPosition {
    Cell(CellState),
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

impl CellState {
    pub fn reverse(self) -> CellState {
        if self == CellState::Empty {
            CellState::Empty
        } else if self == CellState::Red {
            CellState::Blue
        } else {
            CellState::Red
        }
    }

    pub fn get_player(self) -> Option<Player> {
        if self == CellState::Empty {
            None
        } else if self == CellState::Red {
            Some(Player::Red)
        } else {
            Some(Player::Blue)
        }
    }
}

#[derive(Clone)]
pub struct HexBoard {
    pub size: usize,
    pub data: [[CellState; MAX_SIZE]; MAX_SIZE],
    pub empty_count: usize,
}

impl HexBoard {
    pub fn new(size: usize) -> HexBoard {
        HexBoard {
            size,
            data: [[CellState::Empty; MAX_SIZE]; MAX_SIZE],
            empty_count: size * size,
        }
    }

    pub fn rotate180(&self) -> Self {
        let mut result = self.clone();

        for i in 0..self.size {
            for j in 0..self.size - i {
                let temp = result.data[i][j];
                result.data[i][j] = result.data[self.size - i - 1][self.size - j - 1];
                result.data[self.size - i - 1][self.size - j - 1] = temp;
            }
        }

        result
    }

    pub fn fill_row(&mut self, row: usize, s: &str) {
        assert_eq!(s.len(), self.size);
        for (index, c) in s.chars().enumerate() {
            if c == 'r' {
                self.set(row, index, CellState::Red);
            } else if c == 'b' {
                self.set(row, index, CellState::Blue);
            } else {
                self.set(row, index, CellState::Empty);
            }
        }
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == CellState::Empty
    }

    pub fn set_abs(&mut self, pos: usize, value: CellState) {
        self.set(pos / self.size, pos % self.size, value)
    }

    pub fn set(&mut self, x: usize, y: usize, value: CellState) {
        let old_value = self.data[x][y];
        if old_value == CellState::Empty && value != CellState::Empty {
            self.empty_count -= 1;
        } else if old_value != CellState::Empty && value == CellState::Empty {
            self.empty_count += 1;
        }

        self.data[x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.data[x][y]
    }

    pub fn get_abs(&self, pos: usize) -> CellState {
        self.get(pos / self.size, pos % self.size)
    }

    pub fn get_with_padding(&self, x: i32, y: i32) -> CellState {
        if x < 0 || x >= self.size as i32 {
            CellState::Red
        } else if y < 0 || y >= self.size as i32 {
            CellState::Blue
        } else {
            self.get(x as usize, y as usize)
        }
    }

    pub fn get_abs_with_padding(&self, pos: i32) -> CellState {
        self.get_with_padding(pos / self.size as i32, pos % self.size as i32)
    }

    pub fn winner_blue(&self) -> bool {
        let size = self.size as i32;

        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut vis = [[false; MAX_SIZE]; MAX_SIZE];

        for i in 0..self.size {
            if self.get(i, 0) == CellState::Blue {
                stack.push((i as i32, 0));
                vis[i][0] = true;
            }
        }

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            for i in 0..6_usize {
                let (nx, ny) = (x + DIS1[i][0], y + DIS1[i][1]);

                if nx >= 0 && nx < size && ny >= 0 && ny < size && !vis[nx as usize][ny as usize] {
                    vis[nx as usize][ny as usize] = true;
                    let cell = self.get(nx as usize, ny as usize);
                    if cell == CellState::Blue {
                        if ny == size - 1 {
                            return true;
                        }

                        stack.push((nx, ny))
                    }
                }
            }
        }

        false
    }

    pub fn winner_red(&self) -> bool {
        let size = self.size as i32;

        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut vis = [[false; MAX_SIZE]; MAX_SIZE];

        for i in 0..self.size {
            if self.get(0, i) == CellState::Red {
                stack.push((0, i as i32));
                vis[0][i] = true;
            }
        }

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            for i in 0..6_usize {
                let (nx, ny) = (x + DIS1[i][0], y + DIS1[i][1]);

                if nx >= 0 && nx < size && ny >= 0 && ny < size && !vis[nx as usize][ny as usize] {
                    vis[nx as usize][ny as usize] = true;
                    let cell = self.get(nx as usize, ny as usize);
                    if cell == CellState::Red {
                        if nx == size - 1 {
                            return true;
                        }

                        stack.push((nx, ny))
                    }
                }
            }
        }

        false
    }

    /// get winner, make sure there is a winner, no draws
    pub fn winner_definite(&self) -> Player {
        if self.winner_red() {
            Player::Red
        } else {
            Player::Blue
        }
    }

    pub fn winner(&self) -> Option<Player> {
        if self.winner_red() {
            Some(Player::Red)
        } else if self.winner_blue() {
            Some(Player::Blue)
        } else {
            None
        }
    }

    pub fn get_position(&self, x: i32, y: i32) -> BoardPosition {
        let size = self.size as i32;
        if x < 0 {
            BoardPosition::LeftTop
        } else if x >= size {
            BoardPosition::RightBottom
        } else if y < 0 {
            BoardPosition::LeftBottom
        } else if y >= size {
            BoardPosition::RightTop
        } else {
            let cell = self.get(x as usize, y as usize);
            BoardPosition::Cell(cell)
        }
    }

    pub fn get_position_abs(&self, pos: i32) -> BoardPosition {
        let size = self.size as i32;
        self.get_position(pos / size, pos % size)
    }
}

impl Display for HexBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut write_cell = |f: &mut Formatter<'_>, cell: CellState| -> std::fmt::Result {
            match cell {
                CellState::Red => write!(f, "{}", Red.paint("x")),
                CellState::Blue => write!(f, "{}", Blue.paint("x")),
                CellState::Empty => write!(f, "_")
            }
        };

        for x in 0..self.size {
            write!(f, "{}", " ".repeat(x))?;

            for y in 0..self.size {
                write_cell(f, self.get(x, y))?;
                if y != self.size - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::common::board::{CellState, HexBoard};
    use crate::common::player::Player;

    #[test]
    fn test_winner1() {
        let mut board = HexBoard::new(3);
        board.set(0, 0, CellState::Red);
        board.set(1, 0, CellState::Red);
        board.set(2, 0, CellState::Red);

        let winner = board.winner_definite();
        assert_eq!(winner, Player::Red);
    }

    #[test]
    fn test_winner2() {
        let mut board = HexBoard::new(3);
        board.set(0, 0, CellState::Red);
        board.set(1, 0, CellState::Blue);
        board.set(1, 1, CellState::Blue);
        board.set(1, 2, CellState::Blue);
        board.set(2, 0, CellState::Red);

        let winner = board.winner_definite();
        assert_eq!(winner, Player::Blue);
    }

    #[test]
    fn test_winner3() {
        let mut board = HexBoard::new(11);
        board.fill_row(0, "__bbbbbbbbr");
        board.fill_row(1, "rbrrrrrrrbr");
        board.fill_row(2, "r______brbb");
        board.fill_row(3, "______br_b_");
        board.fill_row(4, "______br___");
        board.fill_row(5, "_____rbrrr_");
        board.fill_row(6, "_______br__");
        board.fill_row(7, "______br___");

        let winner = board.winner();
        assert_eq!(winner, None);
    }
}