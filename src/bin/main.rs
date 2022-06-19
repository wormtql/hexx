use hexx::common::board::{CellState, HexBoard};
use hexx::common::player::Player;
use hexx::simulator::save_bridge_simulator::SaveBridgeSimulator;
use hexx::simulator::simulator::Simulator;
use hexx::solver::mcts_solver::MCTSSolver;
use hexx::solver::solver::Solver;

fn main() {
    let mut board = HexBoard::new(11);

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let coords: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
        let (x, y) = (coords[0], coords[1]);

        board.set(x, y, CellState::Red);
        println!("{}", board);

        let solver = MCTSSolver::new(Default::default());
        let (x, y) = solver.solve(&board, Player::Blue);

        board.set(x, y, CellState::Blue);
        println!("{}", board);
    }
    board.set(5, 5, CellState::Red);
    // let simulator = SaveBridgeSimulator;
    //
    // let count = simulator.simulate(&board, Player::Blue, 10000);
    //
    // println!("red wins: {}", count);

    // let solver = MCTSSolver::new(Default::default());
    // let (x, y) = solver.solve(&board, Player::Blue);

    println!("{}", board);
    // println!("{}, {}", x, y);
}
