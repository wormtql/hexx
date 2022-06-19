use crate::common::player::Player;
use crate::utils::string_utils::{invert_string, rotate_string, substitute_string, test_ascii};

/// 00: empty
/// 01: red
/// 10: blue


pub struct CellPattern {
    pub length: usize,
    pub patterns_map: Vec<(bool, usize)>
}

impl CellPattern {
    pub fn new(length: usize) -> CellPattern {
        let size = 1 << (length << 1);
        // println!("{}", size);
        CellPattern {
            length,
            patterns_map: vec![(false, 0); size]
        }
    }

    fn update_helper(&mut self, s: &str, current: usize, critical_index: usize) {
        if s.len() == 0 {
            // println!("{}", critical_index);
            self.patterns_map[current] = (true, critical_index);
            return;
        }

        let c = s.chars().next().unwrap();
        if c == 'r' {
            self.update_helper(&s[1..], (current << 2) | 1, critical_index);
        } else if c == 'b' {
            self.update_helper(&s[1..], (current << 2) | 2, critical_index);
        } else if c == '!' {
            self.update_helper(&s[1..], current << 2, self.length - s.len());
        } else if c == 'x' {
            self.update_helper(&s[1..], current << 2, critical_index);
            self.update_helper(&s[1..], (current << 2) | 1, critical_index);
            self.update_helper(&s[1..], (current << 2) | 2, critical_index);
        }
    }

    pub fn update_with_str(&mut self, s: &str) {
        assert_eq!(s.len(), self.length);

        // for i in 0..s.len() {
        //     let rotated = String::from(&s[i..]) + &s[..i];
            // println!("{}", rotated);
            // self.update_helper(&rotated, 0);
        // }
        self.update_helper(s, 0, usize::MAX);
        // self.update_helper(s, 0, 0);
    }

    pub fn test(&self, value: usize) -> bool {
        self.patterns_map[value].0
    }
}

fn flip_x(s: &str) -> String {
    assert_eq!(s.len(), 8);

    let mut temp: Vec<char> = s.chars().collect();
    temp.swap(1, 7);
    temp.swap(2, 6);
    temp.swap(3, 5);

    temp.iter().collect()
}

fn flip_y(s: &str) -> String {
    assert_eq!(s.len(), 8);

    let mut temp: Vec<char> = s.chars().collect();
    temp.swap(1, 3);
    temp.swap(0, 4);
    temp.swap(7, 5);

    temp.iter().collect()
}

fn load_dead_patterns() -> CellPattern {
    let configs = include_str!("../../share/dead1.txt");
    let mut pattern = CellPattern::new(6);
    for line in configs.lines() {
        for i in 0..6 {
            let rotated = rotate_string(line, i);
            pattern.update_with_str(&rotated);
        }
    }

    pattern
}

fn load_inferior1() -> CellPattern {
    let configs = include_str!("../../share/dead1.txt");
    let mut pattern = CellPattern::new(6);
    // let s = if player == Player::Red { 'r' } else { 'b' };

    for line in configs.lines() {
        for i in 0..6 {
            let rotated = rotate_string(line, i);
            for j in 0..6 {
                if test_ascii(&rotated, j, 'b') || test_ascii(&rotated, j, 'r') {
                    let temp = substitute_string(&rotated, j, '!');
                    pattern.update_with_str(&temp);
                }
            }
        }
    }

    pattern
}

fn load_captured(player: Player) -> CellPattern {
    let configs = include_str!("../../share/captured2.txt");
    let mut pattern = CellPattern::new(8);

    for line in configs.lines() {
        let line2 = if player == Player::Blue {
            invert_string(line, 'r', 'b')
        } else {
            String::from(line)
        };

        let strings = vec![line2.clone(), flip_x(&line2), flip_y(&line2), flip_x(&flip_y(&line2))];

        for item in strings {
            pattern.update_with_str(&item);
        }
    }

    pattern
}

fn load_inferior2(player: Player) -> CellPattern {
    let configs = include_str!("../../share/captured2.txt");
    let mut pattern = CellPattern::new(8);
    let s = if player == Player::Red { 'r' } else { 'b' };

    for line in configs.lines() {
        let line2 = if player == Player::Blue {
            invert_string(line, 'r', 'b')
        } else {
            String::from(line)
        };

        let strings = vec![
            line2.clone(),
            flip_x(&line2),
            flip_y(&line2),
            flip_x(&flip_y(&line2))
        ];

        for item in strings {
            for j in 0..8 {
                if test_ascii(&item, j, s) {
                    let temp = substitute_string(&item, j, '!');
                    pattern.update_with_str(&temp);
                }
            }
        }
    }

    pattern
}

pub struct PatternDict {
    pub dead1: CellPattern,
    pub inferior1: CellPattern,

    pub captured_red2: CellPattern,
    pub captured_blue2: CellPattern,

    // pub inferior_red1: CellPattern,
    // pub inferior_blue1: CellPattern,

    pub inferior_red2: CellPattern,
    pub inferior_blue2: CellPattern
}

impl PatternDict {
    pub fn new() -> PatternDict {
        PatternDict {
            dead1: load_dead_patterns(),
            captured_red2: load_captured(Player::Red),
            captured_blue2: load_captured(Player::Blue),
            // inferior_red1: load_inferior1(),
            // inferior_blue1: load_inferior1(Player::Blue),
            inferior1: load_inferior1(),
            inferior_red2: load_inferior2(Player::Red),
            inferior_blue2: load_inferior2(Player::Blue),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::player::Player;
    use crate::inferior_cell::pattern::{CellPattern, load_captured, load_dead_patterns, load_inferior1, load_inferior2};

    #[test]
    fn test1() {
        let mut p = CellPattern::new(6);
        p.update_with_str("rxxrrr");
        let v = 0b01_00_00_01_01_01;
        assert!(p.test(v));
        let v = 0b01_01_10_01_01_01;
        assert!(p.test(v));
        let v = 0b01_10_00_01_01_01;
        assert!(p.test(v));

        let v = 0b00_10_00_01_01_01;
        assert!(!p.test(v));
    }

    #[test]
    fn test2() {
        let mut p = load_dead_patterns();
        let v = 0b00_00_01_01_01_01;
        assert!(p.test(v));
        let v = 0b01_01_01_10_01_01;
        assert!(p.test(v));
    }

    #[test]
    fn test3() {
        let mut p = load_inferior2(Player::Red);
        let v = 0b_00_00_00_00_00_01_01_01;
        let index = p.patterns_map[v];
        assert!(index.0);
        assert_eq!(index.1, 2);
    }

    #[test]
    fn test_inferior1() {
        let p = load_inferior1();

        let v = 0b_00_00_01_01_01_00;
        assert!(p.test(v));
        let v = 0b_00_01_00_01_01_00;
        assert!(p.test(v));
        let v = 0b_00_10_01_01_01_00;
        assert!(p.test(v));

        // let v = 0b_00_00_00_01_01_00;
        // assert!(!p.test(v));
        // let v = 0b_00_10_01_01_01_10;
        // assert!(!p.test(v));
    }

    #[test]
    fn test_captured2() {
        let p = load_captured(Player::Red);

        let v = 0b_01_00_00_00_01_01_01_01;
        assert!(p.test(v));
        let v = 0b_01_00_00_10_01_01_01_01;
        assert!(p.test(v));
        let v = 0b_01_00_00_10_10_01_01_01;
        assert!(p.test(v));

        let v = 0b_01_00_00_10_00_00_01_01;
        assert!(!p.test(v));
    }

    #[test]
    fn test_captured2_blue() {
        let p = load_captured(Player::Blue);

        let v = 0b_10_00_00_00_10_10_10_10;
        assert!(p.test(v));
        let v = 0b_10_00_00_01_10_10_10_10;
        assert!(p.test(v));
        let v = 0b_10_00_00_01_01_10_10_10;
        assert!(p.test(v));

        let v = 0b_10_00_00_01_00_00_10_10;
        assert!(!p.test(v));
    }

    #[test]
    fn test_inferior2_1() {
        let p = load_inferior2(Player::Red);

        let v = 0b_01_00_00_00_01_01_01_00;
        assert!(p.test(v));
        let v = 0b_01_00_00_10_01_00_01_01;
        assert!(p.test(v));
        let v = 0b_00_00_00_10_10_01_01_01;
        assert!(p.test(v));

        let v = 0b_01_00_00_10_00_00_00_01;
        assert!(!p.test(v));
    }

    #[test]
    fn test_inferior2_2() {
        let p = load_inferior2(Player::Red);

        let v = 0b_01_01_10_01_01_01_00_00;
        assert!(!p.test(v));
        // let v = 0b_01_00_00_10_01_00_01_01;
    }

    #[test]
    fn test_inferior2_3() {
        let p = load_inferior2(Player::Red);

        let v = 0b_00_00_00_10_01_01_01_01;
        assert!(p.test(v));
    }

    #[test]
    fn test_inferior2_4() {
        let p = load_inferior2(Player::Red);

        let v = 0b_10_10_01_01_10_00_00_00;
        assert!(p.test(v));
    }
}