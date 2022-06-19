pub const MAX_SIZE: usize = 19;

pub const DIS1: [[i32; 2]; 6] = [[0, -1], [-1, 0], [-1, 1], [0, 1], [1, 0], [1, -1]];

pub const DIS2_H: [[i32; 2]; 8] = [[0, -1], [-1, 0], [-1, 1], [-1, 2], [0, 2], [1, 1], [1, 0], [1, -1]];
pub const DIS2_V: [[i32; 2]; 8] = [[-1, 0], [-1, 1], [0, 1], [1, 1], [2, 0], [2, -1], [1, -1], [0, -1]];
pub const DIS2_DIAG: [[i32; 2]; 8] = [[-1, 1], [0, 1], [1, 0], [2, -1], [2, -2], [1, -2], [0, -1], [-1, 0]];

pub const DIS_PATTERN_6: [[i32; 2]; 6] = [
    [-1, 0], [-1 ,1], [0, -1], [0, 1], [1, -1], [1, 0]
];

pub const DIS_PATTERN_12: [[i32; 2]; 12] = [
    [-1, 0], [-1 ,1], [0, -1], [0, 1], [1, -1], [1, 0],
    [-2, 1], [-1, 2], [-1, -1], [1, 1], [1, -2], [2, -1]
];