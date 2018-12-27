use std::cmp;
use std::i32;
use std::usize;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    infinite: bool,
    area: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
            infinite: false,
            area: 0,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut points: Vec<Point> = input.lines().map(|x| {
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
    for x in (left - 1)..(right + 2) {
        for y in (top - 1)..(bottom + 2) {
            let mut multiple = false;
            let mut closest_distance = i32::MAX;
            let mut closest = usize::MAX;
            for (n, i) in points.iter().enumerate() {
                let distance = (i.x - x).abs() + (i.y - y).abs();
                if distance < closest_distance {
                    closest_distance = distance;
                    multiple = false;
                    closest = n;
                } else if distance == closest_distance {
                    multiple = true;
                }
            }
            if !multiple {
                points[closest].area += 1;
                if x < left || x > right || y < top || y > bottom {
                    points[closest].infinite = true;
                }
            }
        }
    }
    println!("{}", points.iter().filter(|x| !x.infinite).map(|x| x.area).max().unwrap());
}
