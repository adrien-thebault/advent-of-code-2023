use advent_of_code_2023::*;
use itertools::Itertools;

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day1.txt"));
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            input
                .lines()
                .map(|l| {
                    let digits = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
                    *digits
                        .first()
                        .expect("string should contain at least one digit")
                        * 10
                        + *digits
                            .last()
                            .expect("string should contain at least one digit")
                })
                .sum::<u32>()
        );
    }

    // part 2
    {
        timer!("part 2");
        let string_digits = (1u32..)
            .zip([
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ])
            .collect_vec();
        println!(
            "part 2 : {}",
            input
                .lines()
                .map(|l| {
                    let digits = string_digits
                        .iter()
                        .flat_map(|(digit, as_string)| {
                            [
                                l.find(as_string).map(|pos| (*digit, pos)),
                                l.rfind(as_string).map(|pos| (*digit, pos)),
                            ]
                            .into_iter()
                            .flatten()
                        })
                        .chain(
                            l.chars()
                                .enumerate()
                                .filter_map(|(pos, c)| c.to_digit(10).map(|d| (d, pos))),
                        )
                        .sorted_by(|x, y| x.1.cmp(&y.1))
                        .map(|(digit, _)| digit)
                        .collect_vec();

                    digits
                        .first()
                        .expect("string should contain at least one digit")
                        * 10
                        + digits
                            .last()
                            .expect("string should contain at least one digit")
                })
                .sum::<u32>()
        );
    }
}
