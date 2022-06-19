use std::collections::VecDeque;
use crate::common::board::{CellState, HexBoard};
use crate::common::constants::{DIS1, MAX_SIZE};
use crate::common::player;
use crate::common::player::Player;
use crate::two_distance::graph::Graph;

const ARR_SIZE: usize = MAX_SIZE * MAX_SIZE + 4;

pub fn two_distance(g: &Graph, start: usize, size: usize, out: &mut[usize]) {
    let mut dis = [0x3f3f3f3f; ARR_SIZE];
    let mut sub_dis = [0x3f3f3f3f; ARR_SIZE];
    let mut two_dis = [0x3f3f3f3f; ARR_SIZE];
    let mut vis = [false; ARR_SIZE];

    dis[start] = 0;
    sub_dis[start] = 0;
    two_dis[start] = 0;

    let mut q = VecDeque::with_capacity(ARR_SIZE);
    vis[start] = true;
    q.push_back(start);

    for e in g.iter_edge(start) {
        let to = e.to;
        sub_dis[to] = 0;
        dis[to] = 0;
        two_dis[to] = 1;
        q.push_back(to);
        vis[to] = true;
    }

    // println!("two dis start");
    // SPFA to calculate two distance
    while !q.is_empty() {
        let p = q.pop_front().unwrap();

        vis[p] = false;
        for e in g.iter_edge(p) {
            let to = e.to;
            let d = two_dis[p];
            let mut flag = false;

            if d < dis[to] {
                sub_dis[to] = dis[to];
                dis[to] = d;
                // println!("{:?}", sub_dis);
                two_dis[to] = sub_dis[to] + 1;
                flag = true;
            } else if d < sub_dis[to] {
                sub_dis[to] = d;
                two_dis[to] = d + 1;
                flag = true;
            }

            if flag && !vis[to] {
                q.push_back(to);
                vis[to] = true;
            }
        }
    }
    // println!("two dis fin");

    for i in 0..size * size {
        out[i] = two_dis[i];
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TwoDistanceStart {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

impl HexBoard {
    pub fn two_distance(&self, start: TwoDistanceStart, out: &mut [usize]) {
        let size = self.size;

        let g = self.get_reduced_graph(start);
        two_distance(&g, size * size, size, &mut out[..]);
    }

    pub fn get_reduced_graph(&self, start: TwoDistanceStart) -> Graph {
        let target_player = match start {
            TwoDistanceStart::LeftTop | TwoDistanceStart::RightBottom => Player::Red,
            _ => Player::Blue
        };
        let target_cell = target_player.to_cell();

        let mut g = self.get_reduced_graph_without_border(target_player);
        let mut vis = [[false; MAX_SIZE]; MAX_SIZE];
        let size = self.size;
        let ss = size * size;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        for i in 0..size {
            let (x, y) = match start {
                TwoDistanceStart::LeftTop => (0, i),
                TwoDistanceStart::LeftBottom => (i, 0),
                TwoDistanceStart::RightTop => (i, size - 1),
                TwoDistanceStart::RightBottom => (size - 1, i),
            };
            let pos = x * size + y;

            let cell = self.get(x, y);
            if cell == CellState::Empty {
                g.add_edge(ss, pos);
                g.add_edge(pos, ss);
                vis[x][y] = true;
            } else if cell == target_cell {
                queue.push_back((x, y));
            }
        }

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            for i in 0..6 {
                let (nx, ny) = (x as i32 + DIS1[i][0], y as i32 + DIS1[i][1]);
                if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);

                    let cell = self.get(nx, ny);
                    if cell == CellState::Empty {
                        if !vis[nx][ny] {
                            vis[nx][ny] = true;
                            g.add_edge(nx * size + ny, ss);
                            g.add_edge(ss, nx * size + ny);
                        }
                    } else if cell == target_cell {
                        if !vis[nx][ny] {
                            vis[nx][ny] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        g
    }

    pub fn get_reduced_graph_without_border(&self, player: Player) -> Graph {
        let size = self.size;
        let mut g = Graph::new();
        let target_cell = player.to_cell();

        for x in 0..size {
            for y in 0..size {
                if !self.is_empty(x, y) {
                    continue;
                }

                let mut vis = [[false; MAX_SIZE]; MAX_SIZE];
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_back((x, y));
                vis[x][y] = true;

                while !queue.is_empty() {
                    let (px, py) = queue.pop_front().unwrap();
                    for d in 0..6 {
                        let (nx, ny) = (px as i32 + DIS1[d][0], py as i32 + DIS1[d][1]);

                        if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            let cell = self.get(nx, ny);
                            if cell == CellState::Empty {
                                if !vis[nx][ny] {
                                    vis[nx][ny] = true;
                                    g.add_edge(x * size + y, nx * size + ny);
                                }
                            } else if cell == target_cell {
                                if !vis[nx][ny] {
                                    vis[nx][ny] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }
                }
            }
        }

        g
    }
}

#[cfg(test)]
mod test {
    use crate::common::board::{CellState, HexBoard};
    use crate::two_distance::two_distance::TwoDistanceStart;

    #[test]
    fn test_two_distance_graph1() {
        let mut board = HexBoard::new(5);
        board.set(0, 1, CellState::Red);
        board.set(1, 1, CellState::Red);
        let g = board.get_reduced_graph(TwoDistanceStart::LeftTop);

        assert!(g.is_neighbor(25, 0));
        assert!(!g.is_neighbor(25, 1));
        assert!(g.is_neighbor(25, 5));
        assert!(g.is_neighbor(25, 10));
        assert!(g.is_neighbor(25, 11));
    }

    #[test]
    fn test_two_distance_graph2() {
        let mut board = HexBoard::new(5);
        board.set(1, 2, CellState::Blue);
        board.set(1, 3, CellState::Blue);
        let g = board.get_reduced_graph(TwoDistanceStart::LeftBottom);

        assert!(g.is_neighbor(25, 0));
        assert!(g.is_neighbor(25, 20));
        assert!(g.is_neighbor(6, 9));
        assert!(g.is_neighbor(6, 2));
        assert!(g.is_neighbor(12, 6));
        assert!(g.is_neighbor(13, 3));
    }

    #[test]
    fn test_two_distance1() {
        let mut board = HexBoard::new(5);
        let mut out = [0; 25];
        board.two_distance(TwoDistanceStart::LeftBottom, &mut out[..]);
        // println!("{:?}", out);
        assert_eq!(out, [
            1, 2, 3, 4, 5,
            1, 2, 3, 4, 6,
            1, 2, 3, 5, 7,
            1, 2, 4, 6, 8,
            1, 3, 5, 7, 9
        ]);
    }

    // #[test]
    // fn test_two_distance2() {
    //     let mut board = HexBoard::new(5);
    //     board.set(1, 2, CellState::Blue);
    //     board.set(1, 3, CellState::Blue);
    //     let mut out = [0; 25];
    //     board.two_distance(TwoDistanceStart::LeftBottom, &mut out[..]);
    //     println!("{:?}", out);
    //     assert_eq!(out, [
    //         1, 2, 3, 4, 5,
    //         1, 2, 3, 4, 6,
    //         1, 2, 3, 5, 7,
    //         1, 2, 4, 6, 8,
    //         1, 3, 5, 7, 9
    //     ]);
    // }
}