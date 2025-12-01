// use crate::day1::parser::Move;

// fn part2(input: &str) -> usize {
//     input
//         .split_whitespace()
//         .map(Move::parse)
//         .scan(50u8, |next, y| {
//             let rotation_count = y.rotation_count(*next);
//             *next = y.apply_op(*next);
//             dbg!((&next, rotation_count));

//             Some((*next, rotation_count));
//             todo!();
//         })
//         .count()
// }

// #[test]
// fn it_works() {
//     let input = "
// L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
//     ";
//     let result = part2(input);
//     assert_eq!(result, 4);
// }

// // #[test]
// // fn answer1() {
// //     let input = include_str!("./input.txt");

// //     let result = part2(input);

// //     assert_eq!(result, 1071);
// // }
