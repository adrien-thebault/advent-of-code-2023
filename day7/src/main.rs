use itertools::Itertools;
use std::{iter, str::FromStr};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .filter_map(|l| {
            l.split(' ').next_tuple().and_then(|(c, b)| {
                let mut n = c.chars().counts();
                let (j, (c1, c2)) = (
                    n.remove(&'J').unwrap_or_default(),
                    n.values()
                        .copied()
                        .pad_using(2, |_| 0)
                        .sorted()
                        .rev()
                        .next_tuple()
                        .unwrap_or((0, 0)),
                );

                Some((
                    [c1, c2, j],
                    c.chars()
                        .map(|c| match c {
                            x if c.is_ascii_digit() => x.to_digit(10).unwrap(),
                            'T' => 10,
                            'J' => 11,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => panic!(),
                        } as usize)
                        .collect_vec(),
                    usize::from_str(b).ok()?,
                ))
            })
        })
        .collect_vec();

    // part 1
    println!(
        "part 1 : {}",
        input
            .iter()
            .sorted_by_key(|(n, c, _)| iter::once(
                n.iter()
                    .sorted()
                    .next_tuple()
                    .map(|(_, y, x)| x * 10 + y)
                    .unwrap(),
            )
            .chain(c.iter().copied())
            .collect_vec())
            .enumerate()
            .map(|(i, (_, _, b))| b * (i + 1))
            .sum::<usize>()
    );

    // part 2
    println!(
        "part 2 : {}",
        input
            .iter()
            .sorted_by_key(|([c1, c2, j], c, _)| iter::once((c1 + j) * 10 + c2)
                .chain(c.iter().map(|i| if *i == 11 { 0 } else { *i }))
                .collect_vec())
            .enumerate()
            .map(|(i, (_, _, b))| b * (i + 1))
            .sum::<usize>()
    );
}
