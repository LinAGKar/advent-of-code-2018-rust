use std::cmp;
use std::i32;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let points: Vec<Point> = input.lines().map(|x| {
        let mut coords = x.trim().split(", ");
        Point::new(
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        )
    }).collect();
    let mut left = i32::MAX;
    let mut top = i32::MAX;
    let mut right = i32::MIN;
    let mut bottom = i32::MIN;
    for i in &points {
        left = cmp::min(left, i.x);
        top = cmp::min(top, i.y);
        right = cmp::max(right, i.x);
        bottom = cmp::max(bottom, i.y);
    }
    let mut area = 0;
    for x in (left - 1)..(right + 2) {
        for y in (top - 1)..(bottom + 2) {
            let mut total_distance = 0;
            for i in &points {
                total_distance += (i.x - x).abs() + (i.y - y).abs();
            }
            if total_distance <= 10000 {
                area += 1;
            }
        }
    }
    println!("{}", area);
}
