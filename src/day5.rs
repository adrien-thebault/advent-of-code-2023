use advent_of_code_2023::*;
use itertools::Itertools;
use std::str::FromStr;

macro_rules! go_to {
    ($s: ident, $from: tt, $to: tt, $map: expr) => {
        $map.iter()
            .find(|t| $s >= t.$from && $s < t.$from + t.2)
            .map(|t| $s - t.$from + t.$to)
            .unwrap_or($s)
    };
}

fn main() {
    timer!("total");
    let (seeds, maps);

    {
        timer!("prepare");
        (seeds, maps) = String::from_utf8_lossy(include_bytes!("../inputs/day5.txt"))
            .split_once("\n\n")
            .map(|(seeds, maps)| {
                (
                    seeds
                        .trim_start_matches("seeds: ")
                        .split(' ')
                        .filter_map(|n| usize::from_str(n).ok())
                        .collect_vec(),
                    maps.split("\n\n")
                        .map(|map| {
                            map.lines()
                                .skip(1)
                                .filter_map(|l| {
                                    l.split(' ')
                                        .filter_map(|n| usize::from_str(n).ok())
                                        .collect_tuple::<(_, _, _)>()
                                })
                                .collect_vec()
                        })
                        .collect_vec(),
                )
            })
            .unwrap();
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {:?}",
            seeds
                .iter()
                .map(|s| (0..=6).fold(*s, |acc, i| go_to!(acc, 1, 0, &maps[i])))
                .min()
        );
    }

    // part 2
    {
        timer!("part 2");

        let seeds = seeds
            .chunks(2)
            .filter_map(|c| c.iter().copied().collect_tuple())
            .collect_vec();

        println!(
            "part 2 : {:?}",
            &maps[6]
                .iter()
                .sorted_by(|x1, x2| x1.0.cmp(&x2.0))
                .map(|(dst, _, len)| {
                    (*dst..dst + len)
                        .map(|s| (s, (0..=6).fold(s, |acc, i| go_to!(acc, 0, 1, &maps[6 - i]))))
                        .find(|(_, s)| seeds.iter().any(|(b, e)| s >= b && *s < b + e))
                        .map(|(l, _)| l)
                })
                .find(Option::is_some)
                .flatten()
        );
    }
}
