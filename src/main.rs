use std::{
    array, char,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    time::Instant,
};

fn main() {
    let now = Instant::now();
    day_one();
    let elapsed = now.elapsed();
    println!("day 1: {elapsed:?}");

    day_two();

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

    let now = Instant::now();
    let res = day_five_one(&std::fs::read_to_string("input/day5.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 5 1: {elapsed:?} {res}");

    let now = Instant::now();
    let res = day_five_two(&std::fs::read_to_string("input/day5.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 5 2: {elapsed:?} {res}");

    let now = Instant::now();
    let res = day_six_one(&std::fs::read_to_string("input/day6.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 6 1: {elapsed:?} {res}");

    let now = Instant::now();
    let res = day_six_two(&std::fs::read_to_string("input/day6.txt").unwrap());
    let elapsed = now.elapsed();
    println!("day 6 2: {elapsed:?} {res}");
}

fn day_six_two(inp: &str) -> usize {
    let mut guard_pos = (0, 0);
    let mut visited = HashSet::new();

    let lab: Vec<Vec<Pos>> = inp
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Pos::Dot,
                    '#' => Pos::Obstacle,
                    '^' | 'v' | '<' | '>' => {
                        guard_pos = (i, j);
                        match c {
                            '^' => Pos::Guard(Dir::Up),
                            'v' => Pos::Guard(Dir::Down),
                            '<' => Pos::Guard(Dir::Left),
                            '>' => Pos::Guard(Dir::Right),
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Pos>>()
        })
        .collect();
    let x_len = lab.len();
    let y_len = lab[0].len();
    let mut current_dir = if let Pos::Guard(dir) = lab[guard_pos.0][guard_pos.1] {
        dir
    } else {
        panic!()
    };
    let orig_dir = current_dir;
    let orig_guard_pos = guard_pos;

    loop {
        visited.insert(guard_pos);
        if let Some(new_pos) = current_dir.new_pos(guard_pos, x_len, y_len) {
            if lab[new_pos.0][new_pos.1] == Pos::Obstacle {
                current_dir = current_dir.rotate();
            } else {
                guard_pos = new_pos;
            }
        } else {
            break;
        }
    }

    let mut total = 0;
    // dbg!(visited.len());
    // let mut huh = Vec::new();
    // for x in visited { huh.push(x);}
    // huh.sort();
    // huh.reverse();
    for x in visited {
        if x == orig_guard_pos {
            continue;
        };
        let mut new_visited = HashSet::new();
        let mut prev_rounds: Vec<HashSet<(usize, usize)>> = Vec::new();
        guard_pos = orig_guard_pos;
        current_dir = orig_dir;
        let mut new_lab = lab.clone();
        new_lab[x.0][x.1] = Pos::Obstacle;
        loop {
            if !new_visited.insert(guard_pos) {
                if prev_rounds.contains(&new_visited) {
                    total += 1;
                    break;
                } else {
                    prev_rounds.push(new_visited.clone());
                    new_visited = HashSet::new();
                    new_visited.insert(guard_pos);
                }
            };
            if let Some(new_pos) = current_dir.new_pos(guard_pos, x_len, y_len) {
                if new_lab[new_pos.0][new_pos.1] == Pos::Obstacle {
                    current_dir = current_dir.rotate();
                } else {
                    guard_pos = new_pos;
                }
            } else {
                break;
            }
        }
    }
    total
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn new_pos(
        &self,
        (old_pos_x, old_pos_y): (usize, usize),
        x_len: usize,
        y_len: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Dir::Down if old_pos_x < x_len - 1 => Some((old_pos_x + 1, old_pos_y)),
            Dir::Up if old_pos_x > 0 => Some((old_pos_x - 1, old_pos_y)),
            Dir::Left if old_pos_y > 0 => Some((old_pos_x, old_pos_y - 1)),
            Dir::Right if old_pos_y < y_len - 1 => Some((old_pos_x, old_pos_y + 1)),
            _ => None,
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pos {
    Dot,
    Obstacle,
    Guard(Dir),
}

fn day_six_one(inp: &str) -> usize {
    let mut guard_pos = (0, 0);
    let mut visited = HashSet::new();

    let lab: Vec<Vec<Pos>> = inp
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Pos::Dot,
                    '#' => Pos::Obstacle,
                    '^' | 'v' | '<' | '>' => {
                        guard_pos = (i, j);
                        match c {
                            '^' => Pos::Guard(Dir::Up),
                            'v' => Pos::Guard(Dir::Down),
                            '<' => Pos::Guard(Dir::Left),
                            '>' => Pos::Guard(Dir::Right),
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Pos>>()
        })
        .collect();
    let x_len = lab.len();
    let y_len = lab[0].len();
    let mut current_dir = if let Pos::Guard(dir) = lab[guard_pos.0][guard_pos.1] {
        dir
    } else {
        panic!()
    };

    loop {
        visited.insert(guard_pos);
        if let Some(new_pos) = current_dir.new_pos(guard_pos, x_len, y_len) {
            if lab[new_pos.0][new_pos.1] == Pos::Obstacle {
                current_dir = current_dir.rotate();
            } else {
                guard_pos = new_pos;
            }
        } else {
            break;
        }
    }
    visited.len()
}

#[derive(Clone, Default, PartialEq, Eq)]
struct Page {
    pub after: Vec<usize>,
}

fn day_five_two(inp: &str) -> usize {
    let mut ps = Vec::new();
    ps.resize(100, None);
    let mut total: usize = 0;
    for line in inp.lines() {
        if line.contains('|') {
            let nos: Vec<usize> = line.split('|').map(|x| x.parse().unwrap()).collect();

            ps[nos[1]] = ps[nos[1]]
                .clone()
                .map(|mut x: Page| {
                    x.after.push(nos[0]);
                    x
                })
                .or_else(|| {
                    Some(Page {
                        after: vec![nos[0]],
                    })
                });
        }

        if line.contains(',') {
            let nos = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            let mut safe = true;

            'outer: for i in 0..nos.len() {
                for j in i + 1..nos.len() {
                    if ps[nos[i]]
                        .as_ref()
                        .is_some_and(|x| x.after.contains(&nos[j]))
                    {
                        safe = false;
                        break 'outer;
                    }
                }
            }

            if !safe {
                let nos = permute_day_five(&nos, &[], &ps);
                total += nos[nos.len() / 2];
            }
        }
    }

    total
}

fn permute_day_five(list: &[usize], acc: &[usize], ps: &[Option<Page>]) -> Vec<usize> {
    if list.is_empty() {
        return acc.to_vec();
    }
    let x = list
        .iter()
        .enumerate()
        .find(|(i, x)| {
            let mut nos = list.to_vec();
            nos.remove(*i);
            nos.iter()
                .all(|y| ps[**x].clone().is_some_and(|z| z.after.contains(y)))
        })
        .unwrap();

    let mut new_acc = acc.to_vec();
    new_acc.push(*x.1);
    let mut new_list = list.to_vec();
    new_list.remove(x.0);
    permute_day_five(&new_list, &new_acc, ps)
}

fn day_five_one(inp: &str) -> usize {
    let mut ps = Vec::new();
    ps.resize(100, None);
    let mut total = 0;
    for line in inp.lines() {
        if line.contains('|') {
            let nos: Vec<usize> = line.split('|').map(|x| x.parse().unwrap()).collect();
            ps[nos[1]] = ps[nos[1]]
                .clone()
                .map(|mut x: Page| {
                    x.after.push(nos[0]);
                    x
                })
                .or_else(|| {
                    Some(Page {
                        after: vec![nos[0]],
                    })
                });
        }

        if line.contains(',') {
            let nos = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            let mut safe = true;

            'outer: for i in 0..nos.len() {
                for j in i + 1..nos.len() {
                    if ps[nos[i]]
                        .as_ref()
                        .is_some_and(|x| x.after.contains(&nos[j]))
                    {
                        safe = false;
                        break 'outer;
                    }
                }
            }

            if safe {
                total += nos[nos.len() / 2];
            }
        }
    }

    total
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
    fn day_six() {
        let inp = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let res = day_six_one(inp);
        assert_eq!(res, 41);

        let res = day_six_two(inp);
        assert_eq!(res, 6);
    }

    #[test]
    fn day_five() {
        let res = day_five_one(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(res, 143);

        let res = day_five_two(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(res, 123)
    }

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

#[derive(PartialEq, Eq, Debug)]

enum Order {
    Asc,

    Desc,
}

fn get_order(vec: &[usize]) -> Option<Order> {
    if vec.len() < 3 {
        None
    } else if vec[0] < vec[1] && vec[1] < vec[2] {
        Some(Order::Asc)
    } else if vec[0] > vec[1] && vec[1] > vec[2] {
        Some(Order::Desc)
    } else {
        None
    }
}

fn safe(vec: &[usize]) -> bool {
    match get_order(vec) {
        Some(x) => safe_order(vec, &x),

        None if vec.len() > 2 => false,

        _ => safe_order(vec, &Order::Asc) || safe_order(vec, &Order::Desc),
    }
}

fn safe2(vec: &[usize]) -> bool {
    match get_order(vec) {
        Some(x) => safe2_order(vec, &x),

        _ => safe2_order(vec, &Order::Asc) || safe2_order(vec, &Order::Desc),
    }
}

fn safe_order(vec: &[usize], order: &Order) -> bool {
    let mut safe = true;

    let mut prev = vec[0];

    for n in &vec[1..] {
        let next = *n;

        if unsafe_combo(prev, next, order) {
            safe = false;

            break;
        }

        prev = next;
    }

    safe
}

fn safe2_order(vec: &[usize], order: &Order) -> bool {
    let mut safe = true;

    let mut prev = vec[0];

    let mut skipped: Option<usize> = None;

    let mut i = 0;

    for n in &vec[1..] {
        i += 1;

        let next = *n;

        if unsafe_combo(prev, next, order) {
            if skipped.is_none() {
                skipped = Some(i);

                break;
            }

            safe = false;

            break;
        }

        prev = next;
    }

    if safe && skipped.is_none() {
        return true;
    }

    let problem = skipped.unwrap();

    let mut try1 = vec[problem..].to_vec();

    if problem > 1 {
        try1.insert(0, vec[problem - 2]);
    }

    let try1 = safe_order(&try1, order);

    if try1 {
        return try1;
    }

    let mut try2 = vec[(problem - 1)..].to_vec();

    try2.remove(1);

    safe_order(&try2, order)
}

fn unsafe_combo(prev: usize, next: usize, order: &Order) -> bool {
    (order == &Order::Desc && (prev <= next || (prev - next) > 3))
        || (order == &Order::Asc && (next <= prev || (next - prev) > 3))
}

fn day_two() {
    let now = Instant::now();

    let file = File::open("input/day2.txt").unwrap();

    let res1 = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split(char::is_whitespace)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|x| safe(x))
        .count();

    let elapsed = now.elapsed();

    println!("{res1} -- {elapsed:.2?}");

    let now = Instant::now();

    let file = File::open("input/day2.txt").unwrap();

    let res2 = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split(char::is_whitespace)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|x| safe2(x))
        .count();

    let elapsed = now.elapsed();

    println!("{res2} -- {elapsed:.2?}")
}
