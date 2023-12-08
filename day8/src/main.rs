use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"));
    let (dir, nodes, names) = input
        .split_once("\n\n")
        .map(|(dir, nodes)| {
            let mut names = Vec::with_capacity(nodes.lines().count());
            (
                dir.chars()
                    .map(|c| if c == 'L' { 0usize } else { 1usize })
                    .collect_vec(),
                nodes
                    .split('\n')
                    .filter_map(|n| {
                        n.split_once(" = ").and_then(|(n, d)| {
                            names.push(n);
                            d.trim_matches(&['(', ')'] as &[_]).split(", ").next_tuple()
                        })
                    })
                    .collect_vec()
                    .into_iter()
                    .filter_map(|(l, r)| {
                        names
                            .iter()
                            .position(|n| *n == l)
                            .and_then(|l| names.iter().position(|n| *n == r).map(|r| (l, r)))
                    })
                    .collect_vec(),
                names,
            )
        })
        .unwrap();

    // part 1
    let (start, end) = (
        names.iter().position(|n| *n == "AAA").unwrap(),
        names.iter().position(|n| *n == "ZZZ").unwrap(),
    );
    println!(
        "part 1 : {}",
        dir.iter()
            .cycle()
            .fold_while((1, nodes[start].into()), |(i, acc): (_, [_; 2]), dir| {
                if acc[*dir] == end {
                    Done((i, Default::default()))
                } else {
                    Continue((i + 1, nodes[acc[*dir]].into()))
                }
            })
            .into_inner()
            .0
    );

    // part 2
    println!(
        "part 2 : {}",
        names
            .iter()
            .enumerate()
            .filter(|&(_, n)| n.ends_with('A'))
            .map(|(start, _)| {
                dir.iter()
                    .cycle()
                    .fold_while((1, nodes[start].into()), |(i, acc): (_, [_; 2]), dir| {
                        if names[acc[*dir]].ends_with('Z') {
                            Done((i, Default::default()))
                        } else {
                            Continue((i + 1, nodes[acc[*dir]].into()))
                        }
                    })
                    .into_inner()
                    .0
            })
            .fold(1usize, |acc, x| {
                let pgcd = {
                    let (mut a, mut b) = (acc, x);
                    loop {
                        let r = a - (a / b) * b;
                        if r == 0 {
                            break;
                        }

                        a = b;
                        b = r;
                    }
                    b
                };
                (acc * x) / pgcd
            })
    );
}
