extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::i32;
use std::io;
use std::io::Read;


#[derive(PartialEq, Eq, Hash)]
struct Claim {
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

impl Claim {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Claim {
        Claim {
            x: x,
            y: y,
            x2: x + w,
            y2: y + h,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x <= x && x < self.x2 && self.y <= y && y < self.y2
    }
}

fn main() {
    let mut input = String::new();
    let re = Regex::new(r"(?m)^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut claims = HashSet::new();
    let mut left = i32::MAX;
    let mut top = i32::MAX;
    let mut right = i32::MIN;
    let mut bottom = i32::MIN;
    for i in re.captures_iter(&input) {
        let claim = Claim::new(i[1].parse().unwrap(), i[2].parse().unwrap(), i[3].parse().unwrap(), i[4].parse().unwrap());
        left = cmp::min(left, claim.x);
        top = cmp::min(top, claim.y);
        right = cmp::max(right, claim.x2);
        bottom = cmp::max(bottom, claim.y2);
        claims.insert(claim);
    }
    let mut covered_by_multiple = 0u32;
    for x in left..right {
        for y in top..bottom {
            let mut covered = false;
            for claim in &claims {
                if claim.contains(x, y) {
                    if covered {
                        covered_by_multiple += 1;
                        break;
                    } else {
                        covered = true;
                    }
                }
            }
        }
    }
    println!("{}", covered_by_multiple);
}
