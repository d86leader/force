// https://codeforces.com/problemset/problem/1646/C
use std::vec::Vec;
use std::string::String;

struct Inp {
    // reversed input words for O(1) popping
    words: Vec<String>,
}

impl Inp {
    fn new() -> Self {
        Inp {
            words: Vec::new(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        if let Some(s) = self.words.pop() {
            match s.parse() {
                Ok(x) => x,
                Err(_) => panic!("Parse error")
            }
        } else {
            loop {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                self.words = buf.trim().split_ascii_whitespace().rev().map(|x| x.to_owned()).collect();
                if let Some(s) = self.words.pop() {
                    break match s.parse() {
                        Ok(x) => x,
                        Err(_) => panic!("Parse error")
                    }
                }
            }
        }
    }
}

pub fn solve(n: i64) -> i64 {
    let mut min = usize::MAX;

    fn weight(mut x: u64) -> usize {
        let mut w = 0;
        while x != 0 {
            if x & 1 == 1 {
                w += 1;
            }
            x >>= 1;
        }
        w
    }

    // build a bitmask for a set of all factorials
    let facts_mask_all: u64 = {
        let mut steps = 0;
        let mut f = 2;
        let mut i = 3;
        while f < n {
            f *= i;
            i += 1;
            steps += 1;
        }
        (1 << steps) - 1
    };

    for facts_mask in 0..=facts_mask_all {
        // build a sum of factorials by mask
        let sum = {
            let mut sum = 0;
            let mut f = 6;
            let mut i = 4;
            let mut facts_mask = facts_mask;
            while facts_mask != 0 {
                if facts_mask & 1 == 1 {
                    sum += f;
                }
                f *= i;
                i += 1;
                facts_mask >>= 1;
            }
            sum
        };


        if sum > n {
            continue;
        }
        let n_ = (n - sum).try_into().unwrap();
        let steps = weight(n_) + weight(facts_mask);
        min = std::cmp::min(min, steps);
    }

    min as i64
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let x = inp.read();
        println!("{}", solve(x));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        assert_eq!(crate::solve(7), 2);
        assert_eq!(crate::solve(11), 3);
        assert_eq!(crate::solve(240), 4);
        assert_eq!(crate::solve(17179869184), 1);
        assert_eq!(crate::solve(1), 1);
    }
}
