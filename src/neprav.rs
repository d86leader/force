// https://codeforces.com/problemset/problem/1651/B
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

    fn read_vec<T: std::str::FromStr>(&mut self, size: usize) -> Vec<T> {
        let mut r = Vec::new();
        r.reserve_exact(size);
        for _ in 0..size {
            r.push(self.read());
        }
        r
    }
}

pub fn solve(n: usize) -> Option<Vec<u64>> {
    if n > 19 {
        None
    } else {
        let mut r = Vec::new();
        let mut x = 1;
        for _ in 0..n {
            r.push(x);
            x *= 3;
        }
        Some(r)
    }
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let size = inp.read();
        match solve(size) {
            None => println!("NO"),
            Some(r) => {
                println!("YES");
                for x in r {
                    print!("{} ", x);
                }
                println!();
            },
        }

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        fn counters(v: Vec<u64>) -> bool {
            for (x, y) in v.iter().zip(v.iter()) {
                if x != y {
                    let replaced = 2 * (std::cmp::max(x, y) - std::cmp::min(x, y));
                    let orig = x + y;
                    if replaced < orig {
                        return false;
                    }
                }
            }
            true
        }
        fn check(r: Option<Vec<u64>>, s: bool) {
            match r {
                None if !s => (),
                None       => assert!(false, "expected to find counterexample"),
                Some(v) if s => assert!(counters(v), "bad counterexample"),
                Some(_)      => assert!(false, "expected no counterexample"),
            }
        }
        check(crate::solve(2), true);
        check(crate::solve(512), false);
        check(crate::solve(3), true);
    }
}
