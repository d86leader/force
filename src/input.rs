use std::vec::Vec;
use std::string::String;

pub struct Inp {
    // reversed input words for O(1) popping
    words: Vec<String>,
}

impl Inp {
    pub fn new() -> Self {
        Inp {
            words: Vec::new(),
        }
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> T {
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

    pub fn read_vec<T: std::str::FromStr>(&mut self, size: usize) -> Vec<T> {
        let mut r = Vec::new();
        r.reserve_exact(size);
        for _ in 0..size {
            r.push(self.read());
        }
        r
    }
}
