use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .filter_map(|l| {
            let mut l = vec![l
                .split(' ')
                .filter_map(|n| i32::from_str(n).ok())
                .collect_vec()];

            while !l[l.len() - 1].iter().all(|n| *n == 0) {
                l.push(
                    l[l.len() - 1]
                        .iter()
                        .tuple_windows()
                        .map(|(x, y)| y - x)
                        .collect_vec(),
                );
            }

            l.last_mut()?.extend_from_slice(&[0, 0]);
            for i in (0..l.len() - 1).rev() {
                let x = *(l[i].last()?);
                let y = *(l[i + 1].last()?);
                l[i].push(x + y);

                let x = l[i][0];
                let y = l[i + 1][0];
                l[i].insert(0, x - y);
            }

            Some(l)
        })
        .collect_vec();

    // part 1
    println!(
        "part 1 : {}",
        input
            .iter()
            .filter_map(|l| l.first()?.last().copied())
            .sum::<i32>()
    );

    // part 2
    println!("part 2 : {}", input.iter().map(|l| l[0][0]).sum::<i32>());
}
