// https://codeforces.com/problemset/problem/1650/D
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

pub fn solve(mut a: Vec<i64>) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    for index_target in (0..a.len()).rev() {
        let x: i64 = (index_target + 1).try_into().unwrap();
        let index_current = a.iter().position(|t| *t == x).unwrap();
        let index_current = (index_current + 1) % (index_target + 1);
        let slice = &mut a[0..=index_target];
        slice.rotate_left(index_current);
        result.push(index_current);
    }
    Some(result.into_iter().rev().collect())
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let size = inp.read();
        let mut v = Vec::new();
        v.reserve(size);
        for _ in 0..size {
            v.push(inp.read());
        }
        match solve(v) {
            None => println!("-1"),
            Some(r) => {
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
        assert_eq!(crate::solve(vec![3, 2, 5, 6, 1, 4]), Some(vec![0, 1, 1, 2, 0, 4]));
        assert_eq!(crate::solve(vec![3, 1, 2]), Some(vec![0, 0, 1]));
        assert_eq!(crate::solve(vec![5, 8, 1, 3, 2, 6, 4, 7]), Some(vec![0, 1, 2, 0, 2, 5, 6, 2]));
    }
}
