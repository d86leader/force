// https://codeforces.com/problemset/problem/1649/A
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

pub fn solve(v: Vec<bool>) -> i64 {
    let mut l = v.len() as i64;
    let mut r = 0;
    for i in 0..v.len() {
        if v[i] == false {
            l = i as i64;
            break;
        }
    }
    for i in (0..v.len()).rev() {
        if v[i] == false {
            r = i as i64;
            break;
        }
    }
    if r < l {
        0
    } else {
        r - l + 2
    }
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let size = inp.read();
        let mut location = Vec::new();
        for _ in 0..size {
            let desc: char = inp.read();
            location.push(desc == '1');
        }
        println!("{}", solve(location));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        assert_eq!(crate::solve(vec![true, true]), 0);
        assert_eq!(crate::solve(vec![true, false, true]), 2);
        assert_eq!(crate::solve(vec![true, false, true, false, true]), 4);
        assert_eq!(crate::solve(vec![true, false, true, true]), 2);
    }
}
