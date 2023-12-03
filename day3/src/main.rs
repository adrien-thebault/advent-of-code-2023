use itertools::{Either, Itertools};
use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"));

    let schematic = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let (number_positions, gears_positions): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .peekable()
                .batching(move |it| match it.next() {
                    Some((j, c)) if c.is_digit(10) => {
                        let mut n = c.to_string();
                        while let Some((_, c)) = it.peek() {
                            if c.is_digit(10) {
                                n.push(it.next().unwrap().1);
                            } else {
                                break;
                            }
                        }
                        Some(Some(Either::Left((n, i, j))))
                    }
                    Some((j, '*')) => Some(Some(Either::Right((i, j)))),
                    None => None,
                    _ => Some(None),
                })
                .flatten()
        })
        .partition(|pos| pos.is_left());

    // part 1
    println!(
        "part 1 : {}",
        number_positions
            .clone()
            .into_iter()
            .filter_map(Either::left)
            .filter_map(|(n, i, j)| {
                [(i, j - 1), (i, j + n.len())]
                    .into_iter()
                    .chain(
                        (j as i32 - 1..=j as i32 + n.len() as i32)
                            .filter(|j| *j >= 0)
                            .flat_map(|j| [(i - 1, j as usize), (i + 1, j as usize)].into_iter()),
                    )
                    .any(|(i, j)| {
                        schematic
                            .get(i)
                            .and_then(|l| l.get(j))
                            .is_some_and(|c| !c.is_digit(10) && *c != '.')
                    })
                    .then(|| usize::from_str(&n).unwrap())
            })
            .sum::<usize>()
    );

    // part 2
    let number_positions_by_row: HashMap<_, _> = number_positions
        .into_iter()
        .filter_map(Either::left)
        .group_by(|(_, i, _)| *i)
        .into_iter()
        .map(|(i, group)| (i, group.collect_vec()))
        .collect();

    println!(
        "part 2 : {}",
        gears_positions
            .into_iter()
            .filter_map(Either::right)
            .filter_map(|(row, col)| {
                let adj_numbers = number_positions_by_row
                    .get(&(row - 1))
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .chain(
                        number_positions_by_row
                            .get(&row)
                            .cloned()
                            .unwrap_or_default()
                            .iter(),
                    )
                    .chain(
                        number_positions_by_row
                            .get(&(row + 1))
                            .cloned()
                            .unwrap_or_default()
                            .iter(),
                    )
                    .filter(|&(n, _, j)| {
                        col as i32 >= *j as i32 - 1 && col as i32 <= *j as i32 + n.len() as i32
                    })
                    .map(|(n, _, _j)| usize::from_str(n).unwrap())
                    .collect_vec();

                (adj_numbers.len() == 2)
                    .then(|| adj_numbers.into_iter().reduce(|acc, n| acc * n))
                    .flatten()
            })
            .sum::<usize>()
    )
}
