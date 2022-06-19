use std::cell::{RefCell};
use std::fmt::Display;
use std::rc::{Rc, Weak};
use smallvec::SmallVec;
use crate::common::board::{CellState, HexBoard};
use crate::common::constants::MAX_SIZE;
use crate::common::player::Player;
use crate::cutoff::cutoff::Cutoff;
use crate::cutoff::inferior_cell_cutoff::InferiorCellCutoff;
use crate::cutoff::two_distance_cutoff::TwoDistanceCutoff;
use crate::inferior_cell::inferior_cell::fill_dead_and_captured;
use crate::simulator::save_bridge_simulator::SaveBridgeSimulator;
use crate::simulator::simulator::Simulator;
use crate::solver::solver::Solver;
use serde::Serialize;
use crate::cutoff::pattern_cutoff::PatternCutoff;
use crate::prior::pattern_prior::PatternPrior;
use crate::prior::prior::Prior;

#[derive(Serialize)]
struct Node {
    pub visit: usize,
    pub win: usize,
    pub score: f64,
    // pub amaf_win: [usize; MAX_SIZE * MAX_SIZE],
    // pub amaf_vis: [usize; MAX_SIZE * MAX_SIZE],
    pub amaf_win: usize,
    pub amaf_vis: usize,

    pub prior: f64,
    #[serde(skip_serializing)]
    pub board: HexBoard,
    pub next_player: Player,
    pub player: Player,
    pub mov: usize,
    pub depth: usize,
    pub is_game_over_calculated: bool,
    pub game_over: Option<Player>,
    pub game_over_from_other_methods: Option<Player>,
    pub size: usize,

    // pub parent: Option<Rc<RefCell<Node>>>,
    #[serde(skip_serializing)]
    pub parent: Option<Weak<RefCell<Node>>>,
    #[serde(skip_serializing)]
    pub children: Vec<Rc<RefCell<Node>>>,
    // pub children_count: usize,
}

impl Node {
    fn new(size: usize) -> Node {
        Node {
            visit: 0,
            win: 0,
            score: 0.0,
            amaf_win: 0,
            amaf_vis: 1,
            prior: 0.0,
            board: HexBoard::new(size),
            next_player: Player::Red,
            player: Player::Red,
            mov: 0,
            depth: 0,
            is_game_over_calculated: false,
            game_over: None,
            game_over_from_other_methods: None,
            size,
            parent: None,
            children: Vec::with_capacity(MAX_SIZE * MAX_SIZE),
            // children_count: 0
        }
    }
    
    fn expand_from_node(node: Rc<RefCell<Node>>, position: usize) -> Node {
        let mut board = node.borrow().board.clone();
        let player = node.borrow().next_player;
        board.set_abs(position, player.to_cell());

        Node {
            visit: 0,
            win: 0,
            score: 0.0,
            amaf_win: 0,
            amaf_vis: 0,
            prior: 0.0,
            board,
            next_player: player.reverse(),
            player,
            mov: position,
            depth: node.borrow().depth + 1,
            is_game_over_calculated: false,
            game_over: node.borrow().game_over.clone(),
            game_over_from_other_methods: node.borrow().game_over_from_other_methods.clone(),
            size: node.borrow().size,
            // parent: Some(node.clone()),
            parent: Some(Rc::downgrade(&node)),
            children: Vec::with_capacity(MAX_SIZE * MAX_SIZE)
        }
    }

    fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    fn get_game_over(&mut self) -> Option<Player> {
        if let Some(x) = self.game_over_from_other_methods {
            return Some(x);
        }
        if self.is_game_over_calculated {
            self.game_over.clone()
        } else {
            self.is_game_over_calculated = true;
            self.game_over = self.board.winner();
            self.game_over.clone()
        }
    }
}

#[derive(Clone)]
pub struct MCTSSolverConfig {
    pub min_sim: usize,
    pub max_sim: usize,
    pub simulation_amount: usize,
    pub times_per_sim: usize,

    pub amaf_constant: f64,
    pub ucb_constant: f64,
    pub pb_constant: f64,
    pub win_weight: f64,
}

impl Default for MCTSSolverConfig {
    fn default() -> Self {
        MCTSSolverConfig {
            min_sim: 5,
            max_sim: 10,
            simulation_amount: 500000,
            times_per_sim: 5,
            amaf_constant: 500.0,
            ucb_constant: 1.414,
            pb_constant: 2.0,
            win_weight: 1.0
        }
    }
}

pub struct MCTSSolverHelper {
    pub config: MCTSSolverConfig,

    pub size: usize,
    pub total_expand: usize,
    pub total_cut: usize,
    pub total_nodes: usize,
    pub total_expansion: usize,

    // pub root: Rc<RefCell<Node>>,
    pub simulator: Box<dyn Simulator>,
    pub cutoffs: Vec<Box<dyn Cutoff>>,
    pub prior: Option<Box<dyn Prior>>,
}

impl MCTSSolverHelper {
    fn get_amaf_weight(&self, node: Rc<RefCell<Node>>) -> f64 {
        let k = self.config.amaf_constant;
        let visit = node.borrow().visit as f64;

        // (k / (3.0 * visit + k)).sqrt()
        0.0
    }

    fn get_ucb(&self, node: Rc<RefCell<Node>>) -> f64 {
        let k = self.config.ucb_constant;
        // let parent_visit = node.borrow().parent.as_ref().unwrap().borrow().visit as f64;
        let parent_visit = node.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().visit as f64;
        let visit = node.borrow().visit as f64;

        k * (parent_visit.ln() / visit).sqrt()
    }

    fn get_pb_weight(&self, node: Rc<RefCell<Node>>) -> f64 {
        let k = self.config.pb_constant;
        let visit = node.borrow().visit as f64;
        k / (visit + 1.0).sqrt()
    }

    fn update_mcts(&self, node: Rc<RefCell<Node>>, red_win: usize, total: usize) {
        let mut n: Rc<RefCell<Node>> = node;

        loop {
            // println!("{:?}", n.as_ptr());
            if n.borrow().player == Player::Red {
                n.borrow_mut().win += red_win;
            } else {
                n.borrow_mut().win += total - red_win;
            }
            n.borrow_mut().visit += total;
            // println!("{}", n.borrow().visit);

            let temp = match n.borrow().parent {
                None => {
                    // println!("none");
                    break;
                },
                Some(ref x) => {
                    x.upgrade().unwrap().clone()
                }
            };
            n = temp;

            // n = nn.borrow().parent.clone();
            // n = nn.borrow().parent.as_ref().unwrap().upgrade().clone();
        }
        // println!("loop end");
    }

    fn update_amaf(&self, node: Rc<RefCell<Node>>, red_win: usize, total: usize) {
        let mut n = node;

        // 1: red, 2: blue
        let mut moves = [0; MAX_SIZE * MAX_SIZE];
        let mut moves_count = 0;

        loop {
            let player = n.borrow().player;
            // let mov = n.borrow().mov;
            moves[moves_count] = if player == Player::Red { 1 } else { 2 };
            moves_count += 1;

            for i in 0..moves_count {
                if moves[i] == 1 && player == Player::Red {
                    n.borrow_mut().amaf_win += red_win;
                } else if moves[i] == 2 && player == Player::Blue {
                    n.borrow_mut().amaf_win += total - red_win;
                }
            }
            n.borrow_mut().amaf_vis += total;

            let temp = match n.borrow().parent {
                None => break,
                Some(ref x) => {
                    x.upgrade().unwrap().clone()
                }
            };
            n = temp;
        }
    }

    fn select(&self, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut max_score = -1.0;
        let mut max_index = 0;

        for (i, _) in node.borrow().children.iter().enumerate() {
            let n = node.borrow().children[i].clone();

            // if simulation count < min sim, return the node
            if n.borrow().visit < self.config.min_sim {
                return n;
            }

            let beta = self.get_amaf_weight(n.clone());
            let ucb = self.get_ucb(n.clone());
            let pb = self.get_pb_weight(n.clone());

            let win_weight = self.config.win_weight;
            let win_count = n.borrow().win as f64;
            let visit = n.borrow().visit as f64;

            let amaf_win = n.borrow().amaf_win as f64;
            let amaf_vis = n.borrow().amaf_vis as f64;

            let ucb_part = (win_count * win_weight - (1.0 - win_weight) * (visit - win_count)) / visit;
            let amaf_part = (win_weight * amaf_win - (1.0 - win_weight) * (amaf_vis - amaf_win)) / amaf_vis;

            let score = (1.0 - beta) * ucb_part + beta * amaf_part + ucb
            // let score = ucb_part
                + pb * n.borrow().prior;

            n.borrow_mut().score = score;

            if score > max_score {
                max_score = score;
                max_index = i;
            }
        }

        node.borrow().children[max_index].clone()
    }

    /// return false: no children is available, meaning node is eventually a winning status
    fn expand(&mut self, node: Rc<RefCell<Node>>) -> bool {
        let ss = self.size * self.size;
        let size = self.size;

        let mut cutoffs = [false; MAX_SIZE * MAX_SIZE];

        {
            let board = &node.borrow().board;
            let next_player = node.borrow().next_player;
            let last_move = if node.borrow().mov >= ss {
                None
            } else {
                let temp = node.borrow().mov;
                let x = temp / size;
                let y = temp % size;
                Some((x, y))
            };
            for cutoff in self.cutoffs.iter() {
                cutoff.cutoff(&board, next_player, last_move, &mut cutoffs[..]);
            }
        }

        for i in 0..ss {
            if !cutoffs[i] && node.borrow().board.get_abs(i) == CellState::Empty {
                self.total_expand += 1;

                let new_node = Rc::new(RefCell::new(Node::expand_from_node(node.clone(), i)));
                // let board_with_dead_and_captured = fill_dead_and_captured(
                //     &new_node.borrow().board,
                //     new_node.borrow().next_player
                // );
                // new_node.borrow_mut().board = board_with_dead_and_captured;
                node.borrow_mut().children.push(new_node.clone());

                self.total_nodes += 1;
            }
        }

        self.calc_prior(node.clone());

        self.total_expansion += 1;

        node.borrow().children.len() > 0
    }

    fn calc_prior(&self, node: Rc<RefCell<Node>>) {
        if self.prior.is_none() {
            return;
        }

        let board = &node.borrow().board;
        let size = board.size;
        let mov = node.borrow().mov;
        let last_move = if mov < usize::MAX {
            Some((mov / size, mov % size))
        } else {
            None
        };
        let next_player = node.borrow().next_player;

        let prior = self.prior.as_ref().unwrap();
        let mut out = [0.0; MAX_SIZE * MAX_SIZE];
        prior.prior(&board, last_move, next_player, &mut out[..]);

        for child in node.borrow().children.iter() {
            let mov = child.borrow().mov;
            child.borrow_mut().prior = out[mov];
        }
    }

    fn uct(&mut self, node: Rc<RefCell<Node>>) {
        while node.borrow().visit < self.config.simulation_amount {
            let mut n = node.clone();
            while n.borrow().has_children() {
                n = self.select(n.clone());
            }

            let visit = n.borrow().visit;
            if visit >= self.config.max_sim {
                let is_game_over = n.borrow_mut().get_game_over().is_some();
                if !is_game_over {
                    if !self.expand(n.clone()) {
                        // cannot expand, the game is actually over
                        let player = n.borrow().player;
                        n.borrow_mut().game_over_from_other_methods = Some(player);
                    } else {
                        n = self.select(n.clone());
                    }
                }
            }

            let mut red_win = 0;
            let game_over = n.borrow_mut().get_game_over();
            if let Some(x) = game_over {
                red_win = if x == Player::Red { self.config.times_per_sim } else { 0 };
            } else {
                // not game over
                red_win = self.simulator.simulate(&n.borrow().board, n.borrow().next_player, self.config.times_per_sim);
            }

            // update mcts
            self.update_mcts(n.clone(), red_win, self.config.times_per_sim);
            // update amaf
            self.update_amaf(n.clone(), red_win, self.config.times_per_sim);
        }
    }
}


fn get_most_visited_children(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let mut max_visit = 0;
    let mut max_index = 0_usize;
    for (index, item) in node.borrow().children.iter().enumerate() {
        // let x = serde_json::to_string_pretty(&*item.borrow()).unwrap();
        // println!("{}", x);
        // println!("visit of {}: {}", item.borrow().mov, item.borrow().visit);
        if item.borrow().visit > max_visit {
            max_visit = item.borrow().visit;
            max_index = index;
        }
    }

    node.borrow().children[max_index].clone()
}

pub struct MCTSSolver {
    config: MCTSSolverConfig
}

impl MCTSSolver {
    pub fn new(config: MCTSSolverConfig) -> Self {
        Self {
            config
        }
    }
}

impl Solver for MCTSSolver {
    fn solve(&self, board: &HexBoard, next_player: Player) -> (usize, usize) {
        let mut root = Node::new(board.size);

        root.board = board.clone();
        root.player = next_player.reverse();
        root.next_player = next_player;
        root.mov = usize::MAX;

        let root = Rc::new(RefCell::new(root));

        let mut helper = MCTSSolverHelper {
            config: self.config.clone(),
            size: board.size,
            total_expand: 0,
            total_cut: 0,
            total_nodes: 0,
            total_expansion: 0,
            // root: root.clone(),
            simulator: Box::new(SaveBridgeSimulator),
            cutoffs: vec![
                Box::new(InferiorCellCutoff),
                Box::new(TwoDistanceCutoff { rank: 4 }),
                Box::new(PatternCutoff)
            ],
            // prior: Some(Box::new(PatternPrior))
            prior: None
        };
        helper.uct(root.clone());
        println!("done");

        let most_visited_node = get_most_visited_children(root.clone());
        let mov = most_visited_node.borrow().mov;

        (mov / board.size, mov % board.size)
    }
}