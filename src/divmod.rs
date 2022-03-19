// https://codeforces.com/problemset/problem/1650/B
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

pub fn solve(l: i64, r: i64, a: i64) -> i64 {
    let f = |x: i64| (x / a) + (x % a);
    let x1 = f(r);
    let point = r - (r % a) - 1;
    let point = if point < l {l} else {point};
    let x2 = f(point);
    std::cmp::max(x1, x2)
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let l = inp.read();
        let r = inp.read();
        let a = inp.read();
        println!("{}", solve(l, r, a));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        assert_eq!(crate::solve(1, 4, 3), 2);
        assert_eq!(crate::solve(5, 8, 4), 4);
        assert_eq!(crate::solve(6, 10, 6), 5);
        assert_eq!(crate::solve(1, 1000000000, 1000000000), 999999999);
        assert_eq!(crate::solve(10, 12, 8), 5);
    }
}
