use itertools::Itertools;
use hexx::common::board::HexBoard;
use hexx::common::player::Player;
use hexx::solver::mcts_solver::MCTSSolver;
use hexx::solver::solver::Solver;

fn main() {
    let mut board = HexBoard::new(11);
    // board.fill_row(0, "__bbbbbbbbr");
    // board.fill_row(1, "rbrrrrrrrbr");
    // board.fill_row(2, "r______brb_");
    // board.fill_row(3, "______br_b_");
    // board.fill_row(4, "______br___");
    // board.fill_row(5, "_____rbrrr_");
    // board.fill_row(6, "_______br__");
    // board.fill_row(7, "______br___");

    board.fill_row(3, "______br___");
    board.fill_row(4, "___bb_br___");
    board.fill_row(5, "_b_rrrbrrr_");
    board.fill_row(6, "rbrbrb_bbr_");
    board.fill_row(7, "rbrbr_brr__");
    board.fill_row(8, "rbrbrrr_b__");
    board.fill_row(9, "rb_bbbb____");
    board.fill_row(10, "rb_________");

    println!("{}", board);

    let solver = MCTSSolver::new(Default::default());
    let (x, y) = solver.solve(&board, Player::Blue);

    println!("{}, {}", x, y);
}