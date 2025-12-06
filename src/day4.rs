use advent_of_code_2023::*;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    timer!("total");
    let cards;

    {
        timer!("prepare");
        cards = String::from_utf8_lossy(include_bytes!("../inputs/day4.txt"))
            .lines()
            .filter_map(|l| {
                l.split(':')
                    .nth(1)?
                    .split('|')
                    .map(|n| {
                        n.split(' ')
                            .filter_map(|n| usize::from_str(n).ok())
                            .collect_vec()
                    })
                    .next_tuple()
                    .map(|(w, n)| n.iter().filter(|n| w.contains(n)).count())
            })
            .collect_vec();
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            cards.iter().map(|n| 2u32.pow((n - 1) as u32)).sum::<u32>()
        );
    }

    // part 2
    {
        timer!("part 2");

        let mut copies = vec![1; cards.len()];
        cards.iter().enumerate().for_each(|(c, n)| {
            (c + 1..=c + n).for_each(|i| copies[i] += copies[c]);
        });

        println!("part 2 : {}", copies.iter().sum::<usize>());
    }
}
