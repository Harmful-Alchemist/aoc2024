use std::thread::{self, JoinHandle};

#[derive(Debug)]
struct Line {
    result: usize,
    numbers: Vec<usize>,
}

fn concat(x: usize, y: usize) -> usize {
    x * (10f64.powf(((y+1) as f64).log10().ceil()) as usize) + y
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
        let ops_count = self.numbers.len() - 1;

        for i in 0..(1 << ops_count) {
            let mut acc = self.numbers[0];
            for k in 0..ops_count {
                let j = 1 << k;
                if i & j > 0 {
                    acc += self.numbers[k + 1];
                } else {
                    acc *= self.numbers[k + 1];
                }

                if acc > self.result {
                    break;
                }
            }

            if acc == self.result {
                return self.result;
            }
        }

        0
    }

    fn value2(&self) -> usize {
        let ops_count = self.numbers.len() - 1;
        for i in 0..(3.0_f64.powf(ops_count as f64) as usize) {
            let mut running = i;
            let mut acc = self.numbers[0];
            for i in 0..ops_count {
                match running % 3 {
                    1 => {
                        acc += self.numbers[i + 1];
                    }
                    0 => {
                        acc *= self.numbers[i + 1];
                    }
                    2 => {
                        acc = concat(acc, self.numbers[i + 1]);
                    }
                    _ => panic!(),
                }

                if acc > self.result {
                    break;
                }

                running /= 3;
            }

            if acc == self.result {
                return self.result;
            }
        }

        0
    }
}

pub fn day_seven_one(inp: &str) -> usize {
    inp.lines().filter_map(Line::new).map(|l| l.value()).sum()
}

pub fn day_seven_two(inp: &str) -> usize {
    let mut handles: Vec<JoinHandle<usize>> = Vec::new();
    inp.lines().filter_map(Line::new).for_each(|line| {
        let handle = thread::spawn(move || line.value2());
        handles.push(handle);
    });

    handles.into_iter().map(|h| h.join().unwrap()).sum()

    // inp.lines().filter_map(Line::new).map(|l| l.value2()).sum()
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
