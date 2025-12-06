use advent_of_code_2023::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{iter, str::FromStr};

type Cache = Vec<Vec<(bool, usize)>>;
struct Solver(Cache);

impl Solver {
    pub fn with(r: usize, g: usize, e: usize) -> Self {
        Self(vec![vec![(false, 0); g * e + 1]; 1 + (r + 1) * e])
    }

    pub fn solve(&mut self, record: &[char], groups: &[usize]) -> usize {
        if let (true, r) = self.0[record.len()][groups.len()] {
            r
        } else if !record.is_empty() {
            let r = match record[0] {
                '#' if !groups.is_empty() => {
                    if record.len() >= groups[0]
                        && record[0..groups[0]].iter().all(|&c| c == '?' || c == '#')
                        && (record[groups[0]] == '?' || record[groups[0]] == '.')
                    {
                        self.solve(&record[groups[0] + 1..], &groups[1..])
                    } else {
                        0
                    }
                }
                '?' => {
                    self.solve(&[&['#'], &record[1..]].concat(), groups)
                        + self.solve(&record[1..], groups)
                }
                '.' => self.solve(
                    &record[record.iter().take_while(|&&c| c == '.').count()..],
                    groups,
                ),
                _ => 0,
            };

            self.0[record.len()][groups.len()] = (true, r);
            r
        } else if groups.is_empty() {
            1
        } else {
            0
        }
    }
}

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day12.txt"))
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(record, groups)| {
                (
                    record.to_owned(),
                    groups
                        .split(',')
                        .filter_map(|n| usize::from_str(n).ok())
                        .collect_vec(),
                )
            })
            .collect_vec();
    }

    // part 1
    {
        timer!("part 2");
        println!(
            "part 1 : {}",
            input
                .par_iter()
                .map(
                    |(record, groups)| Solver::with(record.len(), groups.len(), 1)
                        .solve(&record.chars().chain(iter::once('.')).collect_vec(), groups)
                )
                .sum::<usize>()
        );

        // part 2
        println!(
            "part 2 : {}",
            input
                .into_par_iter()
                .map(|(record, groups)| {
                    Solver::with(record.len(), groups.len(), 5).solve(
                        &itertools::repeat_n(record, 5)
                            .join("?")
                            .chars()
                            .chain(iter::once('.'))
                            .collect_vec(),
                        &itertools::repeat_n(groups, 5).flatten().collect_vec(),
                    )
                })
                .sum::<usize>()
        );
    }
}
