use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let (time, dist, races) = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|n| f64::from_str(n).ok())
                .collect_vec()
        })
        .next_tuple()
        .and_then(|(t, d)| {
            Some((
                t.iter().join("").parse::<f64>().ok()?,
                d.iter().join("").parse::<f64>().ok()?,
                t.into_iter().zip(d).collect_vec(),
            ))
        })
        .unwrap();

    let solve = |time: f64, dist: f64| {
        /*
         * d = c*(t-c)
         * <=> 0 = ct - c² - d
         * <=> c² - ct + d = 0
         * => ax² + bx + c = 0 avec a = 1, b = t, c = d
         */
        let s = (
            (-time - f64::sqrt(time * time - 4f64 * 1f64 * dist)) / (2f64 * 1f64),
            (-time + f64::sqrt(time * time - 4f64 * 1f64 * dist)) / (2f64 * 1f64),
        );

        s.1.ceil() - s.0.floor() - 1f64
    };

    // part 1
    println!(
        "part 1 : {}",
        races.into_iter().map(|(t, d)| solve(t, d)).product::<f64>()
    );

    // part2
    println!("part 2 : {}", solve(time, dist));
}
