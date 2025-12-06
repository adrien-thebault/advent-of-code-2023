use advent_of_code_2023::*;
use itertools::Itertools;
use std::cmp;

fn main() {
    timer!("total");
    let (input, empty_rows, empty_cols);

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day11.txt"))
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        (empty_rows, empty_cols) = (
            input
                .iter()
                .enumerate()
                .filter_map(|(x, l)| l.iter().all(|c| *c == '.').then_some(x))
                .collect_vec(),
            (0..input[0].len())
                .filter(|y| input.iter().all(|l| l[*y] == '.'))
                .collect_vec(),
        );
    }

    let solve = |input: &[_], e: usize| {
        input
            .iter()
            .enumerate()
            .flat_map(|(x, l): (_, &Vec<_>)| {
                l.iter()
                    .enumerate()
                    .filter_map(move |(y, c)| (*c == '#').then_some((x, y)))
            })
            .combinations(2)
            .map(|s| {
                let (dx, dy, mx, my) = (
                    s[0].0.abs_diff(s[1].0),
                    s[0].1.abs_diff(s[1].1),
                    cmp::min(s[0].0, s[1].0),
                    cmp::min(s[0].1, s[1].1),
                );
                let empty = empty_rows
                    .iter()
                    .filter(|r| **r > mx && **r < mx + dx)
                    .count()
                    + empty_cols
                        .iter()
                        .filter(|c| **c > my && **c < my + dy)
                        .count();
                dx + dy + empty * (e - 1)
            })
            .sum::<usize>()
    };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", solve(&input, 2));
    }

    // part 2
    {
        timer!("part 2");
        println!("part 2 : {}", solve(&input, 1_000_000));
    }
}
