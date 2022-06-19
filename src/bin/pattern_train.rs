// use std::collections::{HashMap, HashSet};
// use std::fs;
// use itertools::Itertools;
// use hexx::common::board::HexBoard;
// use hexx::common::player::Player;
//
//
// fn load_data() -> Vec<Vec<(Player, usize, usize)>> {
//     let data = fs::read_to_string("./data/data.txt").unwrap();
//     let mut result = Vec::new();
//
//     for line in data.lines() {
//         let mut moves = Vec::new();
//
//         for mov in line.split_whitespace() {
//             let chars = mov.chars().collect_vec();
//             let len = chars.len();
//
//             let player = if chars[0] == 'W' { Player::Blue } else { Player::Red };
//             let y = chars[2] as usize - 'a' as usize;
//             let x = chars[3..len - 1].iter().collect::<String>().parse::<usize>().unwrap() - 1;
//
//             moves.push((player, x, y));
//         }
//
//         result.push(moves);
//     }
//
//     result
// }
//
// fn get_patterns(games: &[Vec<(Player, usize, usize)>]) -> HashSet<usize> {
//     let mut result = HashSet::new();
//     for game in games.iter() {
//         for &(player, x, y) in game.iter() {
//             let mut board = HexBoard::new(13);
//
//             for pattern_item in board.iter_pattern() {
//                 result.insert(pattern_item.encoding);
//             }
//         }
//     }
//
//     result
// }
//
// type PatternEncoding = usize;
//
// fn main() {
//     let data = load_data();
//     let take = 10;
//     println!("game count: {}", data.len());
//
//     let mut winning_count: HashMap<PatternEncoding, usize> = HashMap::new();
//
//     let mut overall_strength: Vec<f64> = Vec::new();
//     let mut overall_strength_composition: Vec<Vec<PatternEncoding>> = Vec::new();
//     let mut cij: HashSet<(PatternEncoding, usize)> = HashSet::new();
//
//     let mut gammas: HashMap<PatternEncoding, f64> = HashMap::new();
//
//     for game in data.iter().take(take) {
//         let mut board = HexBoard::new(13);
//
//         for &(player, x, y) in game.iter() {
//             let winning_pattern = board.get_pattern(x, y).unwrap();
//             let mut patterns_in_this_move: HashSet<PatternEncoding> = HashSet::new();
//
//             for pattern_item in board.iter_pattern() {
//                 patterns_in_this_move.insert(pattern_item.encoding);
//                 gammas.entry(pattern_item.encoding).or_insert(1.0);
//                 winning_count.entry(pattern_item.encoding).or_insert(0);
//             }
//             *winning_count.entry(winning_pattern.encoding).or_insert(0) += 1;
//
//             let mut composition = patterns_in_this_move.iter().cloned().collect_vec();
//             for encoding in composition.iter().cloned() {
//                 cij.insert((encoding, overall_strength_composition.len()));
//             }
//             overall_strength_composition.push(composition);
//
//             board.set(x, y, player.to_cell());
//         }
//     }
//
//     // init overall_strength
//     for (data_index, item) in overall_strength_composition.iter().enumerate() {
//         overall_strength.push(item.len() as f64);
//     }
//
//     // init cij
//     // for encoding in gammas.keys().cloned() {
//     //     for j in 0..overall_strength.len() {
//     //         if overall_strength_composition[j].contains(&encoding) {
//     //             cij.insert((encoding, j));
//     //         }
//     //     }
//     // }
//
//     println!("data count: {}", overall_strength.len());
//     // println!("{:?}", gamma_subscribers.iter().next().unwrap());
//     // println!("{}", winning_count.get(&33372).unwrap());
//
//     println!("pattern count: {:?}", gammas.len());
//
//     let gamma_vec = gammas.keys().cloned().collect_vec();
//
//     let compute_e = |s: &mut [f64], gamma: &HashMap<usize, f64>| {
//         for index in 0..s.len() {
//             let mut new_e = 0.0;
//             for &encoding in overall_strength_composition[index].iter() {
//                 new_e += gamma.get(&encoding).unwrap();
//             }
//
//             s[index] = new_e;
//         }
//     };
//
//     let compute_denominator = |encoding: PatternEncoding, strength: &[f64]| -> f64 {
//         let mut result = 0.0;
//         for j in 0..strength.len() {
//             let c = if cij.contains(&(encoding, j)) { 1.0 } else { 0.0 };
//             result += c / strength[j];
//         }
//
//         result
//     };
//
//     // let mut denominator = compute_denominator()
//
//     for epoch in 0..100 {
//         // let batches = (gamma.len() as f64 / 256.0).ceil() as usize;
//
//         for gamma_index in 0..gamma_vec.len() {
//             // println!("denominator: {}", denominator);
//             let encoding = gamma_vec[gamma_index];
//             let win = *winning_count.get(&encoding).unwrap();
//             // let win = win.max(1);
//             // if win > 0 {
//             //     println!("win of {}: {}", encoding, win);
//             // }
//
//             let old_gamma = *gammas.get(&encoding).unwrap();
//             let new_gamma = if win > 0 {
//                 win as f64 / compute_denominator(encoding, &overall_strength)
//             } else {
//                 0.0
//             };
//
//             gammas.insert(encoding, new_gamma);
//             // for &affected_data_index in gamma_subscribers.get(&encoding).unwrap().iter() {
//             //     let old_e = overall_strength[affected_data_index];
//             //     let new_e = old_e - old_gamma + new_gamma;
//             //     overall_strength[affected_data_index] = new_e;
//             //
//             //     denominator = denominator - 1.0 / old_e + 1.0 / new_e;
//             // }
//
//             println!("{}: {}", encoding, new_gamma);
//         }
//
//         compute_e(&mut overall_strength, &gammas);
//         // denominator = compute_denominator(&overall_strength);
//     }
// }
//
// // fn main() {
// //     let mut gamma: HashMap<usize, f64> = HashMap::new();
// //     let mut winning_count: HashMap<usize, usize> = HashMap::new();
// //     let mut overall_strength: Vec<f64> = Vec::new();
// //     let mut overall_strength_composition: Vec<Vec<usize>> = Vec::new();
// //     let mut cij: Vec<HashMap<usize, >>
// //
// //     let mut gamma_subscribers: HashMap<usize, Vec<usize>> = HashMap::new();
// //
// //     let data = load_data();
// //     println!("game count: {}", data.len());
// //
// //     for game in data.iter().take(100) {
// //         let mut board = HexBoard::new(13);
// //         // println!("game");
// //
// //         for &(player, x, y) in game.iter() {
// //             // let board_r =
// //
// //             let mut patterns_in_this_move = HashSet::new();
// //             let winning_pattern = board.get_pattern(x, y).unwrap();
// //
// //             for pattern_item in board.iter_pattern() {
// //                 patterns_in_this_move.insert(pattern_item.encoding);
// //                 gamma.entry(pattern_item.encoding).or_insert(1.0);
// //                 winning_count.entry(pattern_item.encoding).or_insert(0);
// //             }
// //             *winning_count.entry(winning_pattern.encoding).or_insert(0) += 1;
// //
// //             let mut composition = patterns_in_this_move.iter().cloned().collect_vec();
// //             overall_strength_composition.push(composition);
// //
// //             board.set(x, y, player.to_cell());
// //         }
// //     }
// //
// //     for (data_index, item) in overall_strength_composition.iter().enumerate() {
// //         overall_strength.push(item.len() as f64);
// //
// //         for &gamma_index in item.iter() {
// //             gamma_subscribers.entry(gamma_index).or_insert(Vec::new()).push(data_index);
// //         }
// //     }
// //
// //     println!("data count: {}", overall_strength.len());
// //     // println!("{:?}", gamma_subscribers.iter().next().unwrap());
// //     // println!("{}", winning_count.get(&33372).unwrap());
// //
// //     println!("pattern count: {:?}", gamma.len());
// //
// //     let gamma_vec = gamma.keys().cloned().collect_vec();
// //
// //     let mut denominator = overall_strength.iter().map(|x| 1.0 / *x).sum::<f64>();
// //
// //     let compute_e = |s: &mut [f64], gamma: &HashMap<usize, f64>| {
// //         for index in 0..s.len() {
// //             let mut new_e = 0.0;
// //             for &encoding in overall_strength_composition[index].iter() {
// //                 new_e += gamma.get(&encoding).unwrap();
// //             }
// //
// //             s[index] = new_e;
// //         }
// //     };
// //
// //     let compute_denominator = |arr: &[f64]| -> f64 {
// //         arr.iter().map(|x| 1.0 / *x).sum::<f64>()
// //     };
// //
// //     for epoch in 0..10 {
// //         // let batches = (gamma.len() as f64 / 256.0).ceil() as usize;
// //
// //         for gamma_index in 0..gamma_vec.len() {
// //             // println!("denominator: {}", denominator);
// //             let encoding = gamma_vec[gamma_index];
// //             let win = *winning_count.get(&encoding).unwrap();
// //             // let win = win.max(1);
// //             // if win > 0 {
// //             //     println!("win of {}: {}", encoding, win);
// //             // }
// //
// //             let old_gamma = *gamma.get(&encoding).unwrap();
// //             let new_gamma = win as f64 / denominator;
// //
// //             gamma.insert(encoding, new_gamma);
// //             // for &affected_data_index in gamma_subscribers.get(&encoding).unwrap().iter() {
// //             //     let old_e = overall_strength[affected_data_index];
// //             //     let new_e = old_e - old_gamma + new_gamma;
// //             //     overall_strength[affected_data_index] = new_e;
// //             //
// //             //     denominator = denominator - 1.0 / old_e + 1.0 / new_e;
// //             // }
// //
// //             println!("{}: {}", encoding, new_gamma);
// //         }
// //
// //         compute_e(&mut overall_strength, &gamma);
// //         denominator = compute_denominator(&overall_strength);
// //     }
// //
// //     // for epoch in 0..1 {
// //     //     for gamma_index in 0..gamma_vec.len() {
// //     //         println!("denominator: {}", denominator);
// //     //         let encoding = gamma_vec[gamma_index];
// //     //         let win = *winning_count.get(&encoding).unwrap();
// //     //         let win = win.max(1);
// //     //         // if win > 0 {
// //     //         //     println!("win of {}: {}", encoding, win);
// //     //         // }
// //     //
// //     //         let old_gamma = *gamma.get(&encoding).unwrap();
// //     //         let new_gamma = win as f64 / denominator;
// //     //
// //     //         gamma.insert(encoding, new_gamma);
// //     //         for &affected_data_index in gamma_subscribers.get(&encoding).unwrap().iter() {
// //     //             let old_e = overall_strength[affected_data_index];
// //     //             let new_e = old_e - old_gamma + new_gamma;
// //     //             overall_strength[affected_data_index] = new_e;
// //     //
// //     //             denominator = denominator - 1.0 / old_e + 1.0 / new_e;
// //     //         }
// //     //
// //     //         println!("{}: {}", encoding, new_gamma);
// //     //     }
// //     // }
// // }