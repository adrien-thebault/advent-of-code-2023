use itertools::Itertools;
use std::{cmp, str::FromStr};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .map(|l| l.strip_prefix("Game ").unwrap().split(": "))
        .map(|mut l| {
            (
                usize::from_str(l.next().unwrap()).unwrap(),
                l.next()
                    .unwrap()
                    .split("; ")
                    .map(|round| {
                        round.split(", ").map(|r| r.split(' ')).fold(
                            (0, 0, 0),
                            |(r, g, b), mut iter| {
                                let n = usize::from_str(iter.next().unwrap()).unwrap();
                                match iter.next().unwrap() {
                                    "red" => (r + n, g, b),
                                    "green" => (r, g + n, b),
                                    "blue" => (r, g, b + n),
                                    x => panic!("couleur inconnue : {}", x),
                                }
                            },
                        )
                    })
                    .fold((0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| {
                        (cmp::max(r1, r2), cmp::max(g1, g2), cmp::max(b1, b2))
                    }),
            )
        })
        .collect_vec();

    // part 1
    println!(
        "part 1 : {}",
        input
            .iter()
            .filter_map(|(id, (r, g, b))| (*r < 13 && *g < 14 && *b < 15).then_some(id))
            .sum::<usize>()
    );

    // part 2
    println!(
        "part 2 : {}",
        input.iter().map(|(_, (r, g, b))| r * g * b).sum::<usize>()
    );
}
