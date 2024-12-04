use std::{array, char, iter, time::Instant};

fn main() {
    let now = Instant::now();
    day_one();
    let elapsed = now.elapsed();
    println!("day 1: {elapsed:?}");

    let now = Instant::now();
    let res = day_three_one(&std::fs::read_to_string("input/day3.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 3 1: {elapsed:?} {res}");
    let now = Instant::now();
    let res = day_three_two(&std::fs::read_to_string("input/day3.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 3 2: {elapsed:?} {res}");

    let now = Instant::now();
    let res = day_four_one(&std::fs::read_to_string("input/day4.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 4 1: {elapsed:?} {res}");

    let now = Instant::now();
    let res = day_four_two(&std::fs::read_to_string("input/day4.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 4 2: {elapsed:?} {res}");
}

fn day_four_one(inp: &str) -> usize {
    let to_find = ['X', 'M', 'A', 'S'];
    let cs: Vec<char> = inp.chars().collect();
    let line_len: i32 = line_len(&cs);

    let checks = [
        [0, 1, 2, 3],
        array::from_fn(|x| -(x as i32)),
        array::from_fn(|x| x as i32 * line_len),
        array::from_fn(|x| -(x as i32) * line_len),
        array::from_fn(|x| x as i32 * line_len + x as i32),
        array::from_fn(|x| x as i32 * line_len - x as i32),
        array::from_fn(|x| x as i32 * -line_len + x as i32),
        array::from_fn(|x| x as i32 * -line_len - x as i32),
    ];

    (0..cs.len())
        .map(|i| {
            checks
                .iter()
                .filter(|check| {
                    check.iter().zip(to_find.iter()).all(|(j, c)| {
                        i as i32 + j > -1 && cs.get((i as i32 + j) as usize) == Some(c)
                    })
                })
                .count()
        })
        .sum()
}

fn day_four_two(inp: &str) -> usize {
    let cs: Vec<char> = inp.chars().collect();
    let line_len: i32 = line_len(&cs);
    let (right_top, left_top, right_bottom, left_bottom) =
        (-line_len - 1, -line_len + 1, line_len - 1, line_len + 1);

    (0..cs.len())
        .filter(|i| {
            cs.get(*i) == Some(&'A')
                && is_good_pos(*i, line_len)
                && *i as i32 + right_top > -1
                && check_mas(*i, left_top, right_bottom, &cs)
                && check_mas(*i, right_top, left_bottom, &cs)
        })
        .count()
}

fn line_len(cs: &[char]) -> i32 {
    for (i, c) in cs.iter().enumerate() {
        if *c == '\n' {
            return i as i32 + 1;
        }
    }
    panic!()
}

fn check_mas(i: usize, mod1: i32, mod2: i32, cs: &[char]) -> bool {
    matches!(
        (
            cs.get((i as i32 + mod1) as usize),
            cs.get((i as i32 + mod2) as usize)
        ),
        (Some('M'), Some('S')) | (Some('S'), Some('M'))
    )
}

fn is_good_pos(i: usize, line_len: i32) -> bool {
    let i = i as i32;
    i % line_len != 0 && i % line_len != line_len - 1
}

fn day_three_two(inp: &str) -> i32 {
    let mut new_str = String::new();
    let hmm: Vec<&str> = inp.split("don't()").collect();
    new_str.push_str(hmm[0]);
    hmm.iter()
        .map(|s| s.split("do()").collect())
        .for_each(|split: Vec<&str>| {
            for s in &split[1..] {
                new_str.push_str(s);
            }
        });
    day_three_one(&new_str)
}

fn day_three_one(inp: &str) -> i32 {
    let mut ind = 0;
    let mut lhs = 0;
    let mut rhs = 0;
    let mut total = 0;
    let mut lhs_parse = true;
    let mut number_parsing: bool = false;
    let fns = [
        is('m'),
        is('u'),
        is('l'),
        is('('),
        number(),
        is(','),
        number(),
        is(')'),
    ];
    inp.chars().for_each(|c| match fns[ind](c) {
        PR::Sure => {
            ind += 1;
        }
        PR::No if number_parsing => {
            number_parsing = !number_parsing;
            if let PR::Sure = fns[ind + 1](c) {
                lhs_parse = !lhs_parse;
                ind += 2;

                if ind >= fns.len() {
                    total += lhs * rhs;
                    lhs_parse = true;
                    lhs = 0;
                    rhs = 0;
                    ind = 0;
                }
            } else {
                lhs_parse = true;
                lhs = 0;
                rhs = 0;
                ind = 0;
            }
        }
        PR::No if !number_parsing => {
            lhs_parse = true;
            lhs = 0;
            rhs = 0;
            ind = 0;
        }
        PR::Number(x) => {
            number_parsing = true;
            if lhs_parse {
                lhs = 10 * lhs + x;
            } else {
                rhs = 10 * rhs + x;
            }
        }
        _ => panic!(),
    });
    total
}

enum PR {
    Sure,
    No,
    Number(i32),
}

fn is(check: char) -> Box<dyn Fn(char) -> PR> {
    Box::new(move |c| if c == check { PR::Sure } else { PR::No })
}

fn number() -> Box<dyn Fn(char) -> PR> {
    Box::new(move |c| {
        if c.is_numeric() {
            PR::Number(c.to_string().parse::<i32>().unwrap())
        } else {
            PR::No
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_four() {
        let result = day_four_one(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        assert_eq!(result, 18);

        let result = day_four_two(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );

        assert_eq!(result, 9);
    }

    #[test]
    fn day_three() {
        let result = day_three_one(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(result, 161);

        let result = day_three_two(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+don't()mul(32,64](mul(11,8)undo()?do()mul(8,5))",
        );
        assert_eq!(result, 48);
    }
}

fn day_one() {
    let mut res1 = std::fs::read_to_string("input/day1.txt")
        .unwrap()
        .lines()
        .map(|l| l.split("   "))
        .fold((Vec::new(), Vec::new()), |mut acc, mut t| {
            acc.0.push(t.next().unwrap().trim().parse::<i64>().unwrap());
            acc.1.push(t.next().unwrap().trim().parse::<i64>().unwrap());
            acc
        });

    res1.0.sort();
    res1.1.sort();
    let max = *res1.1.last().unwrap();
    let mut total = 0;

    for x in res1.0.clone().into_iter().enumerate() {
        total += i64::abs(x.1 - res1.1[x.0]);
    }

    println!("{total}");

    let mut counts: Vec<usize> = iter::repeat(0).take(max as usize + 1).collect();

    for x in res1.1 {
        counts[x as usize] += 1
    }

    let lol = res1
        .0
        .iter()
        .map(|x| {
            let x = *x as usize;
            x * counts[x]
        })
        .sum::<usize>();

    println!("{lol}")
}
