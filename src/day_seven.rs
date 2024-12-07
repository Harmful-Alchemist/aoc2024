#[derive(Debug)]
struct Line {
    result: usize,
    numbers: Vec<usize>,
}

fn add(x: usize, y: usize) -> usize {
    x + y
}

fn mul(x: usize, y: usize) -> usize {
    x * y
}

fn concat(x: usize, y: usize) -> usize {
    let mut x = x.to_string();
    x.push_str(&y.to_string());
    x.parse().unwrap()
}

impl Line {
    fn new(inp: &str) -> Option<Line> {
        let parts: Vec<&str> = inp.split(':').collect();
        parts
            .first()
            .map(|s| s.trim().parse::<usize>().unwrap())
            .and_then(|result| {
                parts
                    .get(1)
                    .map(|p| {
                        p.trim()
                            .split(' ')
                            .map(|c| c.parse::<usize>().unwrap())
                            .collect()
                    })
                    .map(|numbers| Line { result, numbers })
            })
    }

    fn value(&self) -> usize {
        let mut ops_perms = Vec::new();
        let ops_count = self.numbers.len() - 1;

        for i in 0..(1 << ops_count) {
            let mut ops: Vec<fn(usize, usize) -> usize> = Vec::new();
            for k in 0..ops_count {
                let j = 1 << k;
                if i & j > 0 {
                    ops.push(add);
                } else {
                    ops.push(mul);
                }
            }
            ops_perms.push(ops);
        }

        self.fun_name(ops_perms)
    }

    fn fun_name(&self, ops_perms: Vec<Vec<fn(usize, usize) -> usize>>) -> usize {
        if ops_perms
            .into_iter()
            .map(|ops| {
                ops.iter()
                    .enumerate()
                    .fold(self.numbers[0], |acc, (i, op)| {
                        if acc > self.result {
                            acc
                        } else {
                            op(acc, self.numbers[i + 1])
                        }
                    })
            })
            .any(|x| x == self.result)
        {
            self.result
        } else {
            0
        }
    }

    fn value2(&self) -> usize {
        let ops_count = self.numbers.len() - 1;

        for i in 0..(3.0_f64.powf(ops_count as f64) as usize) {
            let mut ops: Vec<fn(usize, usize) -> usize> = Vec::new();
            let mut running = i;
            for _ in 0..ops_count {
                match running % 3 {
                    0 => ops.push(add),
                    1 => ops.push(mul),
                    2 => ops.push(concat),
                    _ => panic!(),
                }
                running /= 3;
            }

            let res = ops.iter()
            .enumerate()
            .fold(self.numbers[0], |acc, (i, op)| {
                if acc > self.result {
                    acc
                } else {
                    op(acc, self.numbers[i + 1])
                }
            });
            if res == self.result {
                return  self.result;
            }
        }

        0
    }
}

pub fn day_seven_one(inp: &str) -> usize {
    inp.lines().filter_map(Line::new).map(|l| l.value()).sum()
}

pub fn day_seven_two(inp: &str) -> usize {
    //354060705047464
    inp.lines().filter_map(Line::new).map(|l| l.value2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_seven() {
        let inp = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
100000: 10 10 10 10 10";

        let res = day_seven_one(inp);
        assert_eq!(res, 103749);

        let inp = "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";

        let res = day_seven_two(inp);
        assert_eq!(res, 11387);
    }
}
