use hexx::common::board::{CellState, HexBoard};
use hexx::common::player::Player;
use hexx::simulator::save_bridge_simulator::SaveBridgeSimulator;
use hexx::simulator::simulator::Simulator;
use hexx::solver::mcts_solver::MCTSSolver;
use hexx::solver::solver::Solver;

fn main() {
    let mut board = HexBoard::new(11);
    board.set(5, 5, CellState::Red);

    let solver = MCTSSolver::new(Default::default());
    let (x, y) = solver.solve(&board, Player::Blue);

    println!("{}, {}", x, y);
}
