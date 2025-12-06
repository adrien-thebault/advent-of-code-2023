use advent_of_code_2023::*;
use itertools::Itertools;
use std::cmp;

fn main() {
    timer!("total");
    let mut input = String::from_utf8_lossy(include_bytes!("../inputs/day13.txt"))
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .collect_vec();

    let v_split = |p: Vec<Vec<_>>| {
        (0..p[0].len())
            .position(|c| {
                p.iter()
                    .all(|l| (0..=cmp::min(p[0].len() - c, c)).all(|w| l[c - w] == l[c + w + 1]))
            })
            .map(|c| c + 1)
    };

    // let h_split = |p: Vec<Vec<_>>| {
    //     (0..p.len() - 1)
    //         .position(|l| {
    //             p[l] == p[l + 1] && (1..cmp::min(p.len() - l, l)).all(|h| p[l - h] == p[l + h + 1])
    //         })
    //         .map(|l| l + 1)
    // };

    dbg!(v_split(input.remove(0)));
    // dbg!(h_split(input.remove(0)));

    // part 1
    // println!("part 1 : {}"
    //     input.iter().map(|p| {
    //         let col = p[0].len() / 2;

    //     })
    // )
}
