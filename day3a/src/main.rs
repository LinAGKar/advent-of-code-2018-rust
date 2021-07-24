use regex::Regex;
use std::cmp;
use std::io::{Read,stdin};

struct Rect {
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Rect {
        Rect {
            x: x,
            y: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

fn main() {
    let mut input = String::new();
    let re = Regex::new(r"(?m)^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    stdin().read_to_string(&mut input).unwrap();
    let mut claims = Vec::new();
    let mut left = usize::MAX;
    let mut top = usize::MAX;
    let mut right = 0;
    let mut bottom = 0;
    for i in re.captures_iter(&input) {
        let claim = Rect::new(i[1].parse().unwrap(), i[2].parse().unwrap(), i[3].parse().unwrap(), i[4].parse().unwrap());
        left = cmp::min(left, claim.x);
        top = cmp::min(top, claim.y);
        right = cmp::max(right, claim.x2);
        bottom = cmp::max(bottom, claim.y2);
        claims.push(claim);
    }

    let mut map = vec![vec![false; bottom - top]; right - left];

    println!("{}", claims.iter().enumerate().flat_map(|(n, i)| {
        claims.iter().enumerate().filter(move |&(m, _)| m != n).map(move |(_, j)| (i, j))
    }).map(|(i, j)| {
        if i.x < j.x2 && j.x < i.x2 && i.y < j.y2 && j.y < i.y2 {
            let intersect = Rect {
                x: cmp::max(i.x, j.x),
                x2: cmp::min(i.x2, j.x2),
                y: cmp::max(i.y, j.y),
                y2: cmp::min(i.y2, j.y2),
            };
            (intersect.x..intersect.x2)
                .flat_map(|x| (intersect.y..intersect.y2).map(move |y| (x, y)))
                .filter(|(x, y)| {
                    let new = !map[x - left][y - top];
                    map[x - left][y - top] = true;
                    new
                })
                .count()
        } else {
            0
        }
    }).sum::<usize>());
}
