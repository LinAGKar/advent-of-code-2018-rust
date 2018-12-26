extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::Read;


#[derive(PartialEq, Eq, Hash)]
struct Claim {
    id: String,
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

impl<'a> Claim {
    fn new(id: &'a str, x: i32, y: i32, w: i32, h: i32) -> Claim {
        Claim {
            id: id.to_string(),
            x: x,
            y: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

fn main() {
    let mut input = String::new();
    let re = Regex::new(r"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut claims = HashSet::new();
    for i in re.captures_iter(&input) {
        claims.insert(Claim::new(&i[1], i[2].parse().unwrap(), i[3].parse().unwrap(), i[4].parse().unwrap(), i[5].parse().unwrap()));
    }
    'outer_loop: for claim in claims.iter() {
        for claim2 in claims.iter() {
            if claim.id != claim2.id && claim.x < claim2.x2 && claim2.x < claim.x2 && claim.y < claim2.y2 && claim2.y < claim.y2 {
                continue 'outer_loop;
            }
        }
        println!("{}", claim.id);
        break;
    }
}
