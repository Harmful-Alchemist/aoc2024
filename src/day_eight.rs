use std::collections::{HashMap, HashSet};

pub fn day_eight_one(inp: &str) -> usize {
    solve(inp, antinodes)
}

pub fn day_eight_two(inp: &str) -> usize {
    solve(inp, antinodes2)
}

type AntiNodeFn = fn(
    locationn_i: usize,
    locationn_j: usize,
    locationm_i: usize,
    locationm_j: usize,
    i_len: usize,
    j_len: usize,
) -> Vec<(usize, usize)>;

fn solve(inp: &str, antinodes: AntiNodeFn) -> usize {
    let mut antennas = HashMap::new();
    let mut i_len = 0;
    let mut j_len = 0;
    inp.lines()
        .enumerate()
        .filter_map(|x| {
            if x.1.is_empty() {
                None
            } else {
                Some((x.0, x.1.trim()))
            }
        })
        .for_each(|(i, l)| {
            i_len += 1;
            j_len = l.len();
            l.chars().enumerate().for_each(|(j, c)| match c {
                '.' => (),
                x => {
                    antennas
                        .entry(x)
                        .and_modify(|v: &mut Vec<(usize, usize)>| v.push((i, j)))
                        .or_insert(vec![(i, j)]);
                }
            });
        });

    let mut total = HashSet::new();
    for (_, locations) in antennas {
        for n in 0..locations.len() - 1 {
            for m in n + 1..locations.len() {
                let (locationn_i, locationn_j) = locations[n];
                let (locationm_i, locationm_j) = locations[m];

                for antinode in antinodes(
                    locationn_i,
                    locationn_j,
                    locationm_i,
                    locationm_j,
                    i_len,
                    j_len,
                ) {
                    total.insert(antinode);
                }
            }
        }
    }
    total.len()
}

fn antinodes(
    locationn_i: usize,
    locationn_j: usize,
    locationm_i: usize,
    locationm_j: usize,
    i_len: usize,
    j_len: usize,
) -> Vec<(usize, usize)> {
    let mut total = Vec::new();
    assert!(locationn_i <= locationm_i);
    let i_diff = locationm_i - locationn_i;
    if locationn_j < locationm_j {
        let j_diff = locationm_j - locationn_j;
        if i_diff <= locationn_i && j_diff <= locationn_j {
            let pos = (locationn_i - i_diff, locationn_j - j_diff);
            total.push(pos);
        }
        if locationm_j + j_diff < j_len && locationm_i + i_diff < i_len {
            let pos = (locationm_i + i_diff, locationm_j + j_diff);
            total.push(pos);
        }
    } else {
        let j_diff = locationn_j - locationm_j;
        if i_diff <= locationn_i && locationn_j + j_diff < j_len {
            let pos = (locationn_i - i_diff, locationn_j + j_diff);
            total.push(pos);
        }
        if locationm_j >= j_diff && locationm_i + i_diff < i_len {
            let pos = (locationm_i + i_diff, locationm_j - j_diff);
            total.push(pos);
        }
    }

    total
}

fn antinodes2(
    locationn_i: usize,
    locationn_j: usize,
    locationm_i: usize,
    locationm_j: usize,
    i_len: usize,
    j_len: usize,
) -> Vec<(usize, usize)> {
    let mut total = Vec::new();
    assert!(locationn_i <= locationm_i);
    let orig_i_diff = locationm_i - locationn_i;
    if locationn_j < locationm_j {
        let orig_j_diff = locationm_j - locationn_j;
        let mut j_diff = 0;
        let mut i_diff = 0;
        loop {
            if i_diff <= locationn_i && j_diff <= locationn_j {
                let pos = (locationn_i - i_diff, locationn_j - j_diff);
                total.push(pos);
            }
            if locationm_j + j_diff < j_len && locationm_i + i_diff < i_len {
                let pos = (locationm_i + i_diff, locationm_j + j_diff);
                total.push(pos);
            }
            j_diff += orig_j_diff;
            i_diff += orig_i_diff;
            if j_diff > j_len || i_diff > i_len {
                break;
            }
        }
    } else {
        let orig_j_diff = locationn_j - locationm_j;
        let mut j_diff = 0;
        let mut i_diff = 0;
        loop {
            if i_diff <= locationn_i && locationn_j + j_diff < j_len {
                let pos = (locationn_i - i_diff, locationn_j + j_diff);
                total.push(pos);
            }
            if locationm_j >= j_diff && locationm_i + i_diff < i_len {
                let pos = (locationm_i + i_diff, locationm_j - j_diff);
                total.push(pos);
            }
            j_diff += orig_j_diff;
            i_diff += orig_i_diff;
            if j_diff > j_len || i_diff > i_len {
                break;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_eight() {
        let inp = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let res = day_eight_one(inp);
        assert_eq!(res, 14);

        let res = day_eight_two(inp);
        assert_eq!(res, 34);
    }
}
