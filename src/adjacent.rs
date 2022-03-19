// https://codeforces.com/problemset/problem/1650/A
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

pub fn solve(s: String, c: char) -> bool {
    let l = s.len();
    for (i, x) in s.chars().enumerate() {
        if x == c {
            if i % 2 == 0 && (l - i - 1) % 2 == 0 {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let s = inp.read();
        let c = inp.read();
        if solve(s, c) {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        assert_eq!(crate::solve("abcde".to_owned(), 'c'), true);
        assert_eq!(crate::solve("abcde".to_owned(), 'b'), false);
        assert_eq!(crate::solve("x".to_owned(), 'y'), false);
        assert_eq!(crate::solve("aaaaaaaaaaaaaaa".to_owned(), 'a'), true);
        assert_eq!(crate::solve("contest".to_owned(), 't'), true);
    }
}
