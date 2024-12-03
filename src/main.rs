use std::{iter};

fn main() {
    day_one();
    let res = day_three_one(&std::fs::read_to_string("input/day3.txt").unwrap());
    println!("{res}");
    let res = day_three_two(&std::fs::read_to_string("input/day3.txt").unwrap());
    println!("{res}");
}

fn day_three_two(inp: &str) -> i32 {
    let mut new_str = String::new();
    let hmm: Vec<&str> = inp.split("don't()").collect();
    new_str.push_str(hmm[0]);
    hmm.iter()
        .map(|s| s.split("do()").collect())
        .for_each(|split: Vec<&str>| {
            for s in &split[1..] {
                new_str.push_str(*s);
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
        .fold(0, |acc, x| acc + x);

    println!("{lol}")
}
