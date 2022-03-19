// https://codeforces.com/problemset/problem/1650/C
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

#[derive(Debug, Clone)]
pub struct Point {
    coord: i64,
    weight: i64,
}
pub fn solve(size: usize, points: Vec<Point>) -> (Vec<(usize, usize)>, i64) {
    let mut indicies: Vec<usize> = (0..points.len()).collect();

    indicies.sort_unstable_by_key(|i| points[*i].weight);
    let lightest = &mut indicies[0..size * 2];

    let weight: i64 = lightest.iter().map(|i| points[*i].weight).sum();

    lightest.sort_unstable_by_key(|i| points[*i].coord);
    let mut result = Vec::new();
    for i in 0..size {
        result.push( (lightest[i], lightest[size*2 - i - 1]) )
    }

    (result, weight)
}

fn main() {
    let mut inp = Inp::new();

    let tries = inp.read();
    for _ in 0..tries {
        let size = inp.read();
        let amount = inp.read();
        let mut points = Vec::new();
        for _ in 0..amount {
            let coord = inp.read();
            let weight = inp.read();
            points.push(Point {coord, weight});
        }
        let (r, weight) = solve(size, points);
        println!("{}", weight);
        for (l, r) in r {
            println!("{} {}", l+1, r+1);
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn given() {
        fn p(c: i64, w: i64) -> crate::Point {
            crate::Point{coord: c, weight: w}
        }

        {
            let points = vec![p(0, 10), p(-2, 1), p(4, 10), p(11, 20), p(7, -1), p(9, 1), p(2, 3), p(5, -2)];
            let (r, w) = crate::solve(3, points.clone());
            assert_eq!(w, 12);
            for (l, r) in r {
                assert!(points[l].coord < points[r].coord);
            }
        }
    }
}
