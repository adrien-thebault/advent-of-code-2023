use colored::*;
use itertools::Itertools;
use std::iter;

fn main() {
    let mut input = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .map(|l| {
            iter::once('X')
                .chain(l.chars())
                .chain(iter::once('X'))
                .collect_vec()
        })
        .collect_vec();

    input.insert(0, vec!['X'; input[0].len()]);
    input.push(vec!['X'; input[0].len()]);

    let (sx, sy) = input
        .iter()
        .enumerate()
        .find_map(|(x, l)| l.iter().position(|c| *c == 'S').map(|y| (x, y)))
        .unwrap();

    match (
        ['-', 'J', '7'].contains(&input[sx][sy + 1]),
        ['-', 'L', 'F'].contains(&input[sx][sy - 1]),
        ['|', 'F', '7'].contains(&input[sx - 1][sy]),
        ['|', 'L', 'J'].contains(&input[sx + 1][sy]),
    ) {
        (true, true, false, false) => input[sx][sy] = '-',
        (true, false, true, false) => input[sx][sy] = 'L',
        (true, false, false, true) => input[sx][sy] = 'F',
        (false, true, true, false) => input[sx][sy] = 'J',
        (false, true, false, true) => input[sx][sy] = '7',
        (false, false, true, true) => input[sx][sy] = '|',
        _ => panic!(),
    }

    // part 1
    let (mut d, mut x, mut y, mut px, mut py, mut path) = (1, sx, sy, sx, sy, vec![]);
    loop {
        path.push((x, y));
        let (nx, ny) = match input[x][y] {
            '|' => [(x - 1, y), (x + 1, y)],
            '-' => [(x, y - 1), (x, y + 1)],
            'L' => [(x - 1, y), (x, y + 1)],
            'J' => [(x - 1, y), (x, y - 1)],
            '7' => [(x + 1, y), (x, y - 1)],
            'F' => [(x + 1, y), (x, y + 1)],
            _ => panic!(),
        }
        .into_iter()
        .find(|(nx, ny)| !(*nx == px && *ny == py))
        .unwrap();

        if (nx, ny) == (sx, sy) {
            break;
        } else {
            px = x;
            py = y;
            x = nx;
            y = ny;
            d += 1;
        }
    }
    println!("part 1 : {}", d / 2);

    // part 2
    println!(
        "part 2 : {}",
        input
            .iter()
            .enumerate()
            .map(|(x, l)| {
                let mut inside = false;
                l.iter()
                    .enumerate()
                    .map(|(y, c)| {
                        if path.contains(&(x, y)) {
                            if ['|', 'L', 'J'].contains(c) {
                                inside = !inside;
                            }
                            0
                        } else if inside {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    );

    // bonus : print map for fun
    input.iter().enumerate().for_each(|(x, l)| {
        let mut inside = false;
        l.iter().enumerate().for_each(|(y, c)| {
            let c = if path.contains(&(x, y)) {
                if ['|', 'L', 'J'].contains(c) {
                    inside = !inside;
                }
                match input[x][y] {
                    'L' => "└",
                    'J' => "┘",
                    'F' => "┌",
                    '7' => "┐",
                    '-' => "─",
                    '|' => "│",
                    _ => panic!(),
                }
                .white()
            } else if inside {
                ".".green()
            } else {
                ".".red()
            };

            print!("{c}");
        });
        println!();
    });
}
