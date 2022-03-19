mod input;

use std::vec::Vec;
use std::string::String;


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
    let mut inp = input::Inp::new();

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
