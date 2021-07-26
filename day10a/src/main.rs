use regex::Regex;
use std::io::{Read,stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$").unwrap();

    let points: Vec<(f64, f64, f64, f64)> = re.captures_iter(&input).map(|x| {(
        x[1].parse().unwrap(),
        x[2].parse().unwrap(),
        x[3].parse().unwrap(),
        x[4].parse().unwrap(),
    )}).collect();

    let sum = points.iter().copied().reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)).unwrap();
    let len = points.len() as f64;
    let avg = (sum.0 / len, sum.1 / len, sum.2 / len, sum.3 / len);
    let points: Vec<_> = points.into_iter().map(|(x, y, dx, dy)| {
        (x - avg.0, y - avg.1, dx - avg.2, dy - avg.3)
    }).collect();

    let intermediates: Vec<_> = points.iter().map(|&(x, y, dx, dy)| (
        x * dx + y * dy,
        dx * dx + dy * dy,
        x * x + y * y,
        2.0 * x * dx + y * dy,
    )).collect();

    let dist_sum_prim = move |t: f64| intermediates.iter().map(|vals| {
        (vals.0 + t * vals.1) / (vals.2 + t * vals.3 + t * t * vals.1).sqrt()
    }).sum::<f64>();

    let mut start = 0.0;
    let mut end = 1.0;
    while dist_sum_prim(end) < 0.0 {
        end *= 2.0;
    }

    loop {
        let val = start + (end - start) / 2.0;
        if dist_sum_prim(val) > 0.0 {
            end = val;
        } else {
            start = val;
        }
        if end - start < 1.0 {
            break;
        }
    }

    let possible: std::collections::HashSet<_> = [
        start.floor() as u16, start.ceil() as u16,
        end.floor() as u16, end.ceil() as u16,
    ].iter().copied().collect();

    let t = possible.into_iter().map(|t| t as f64).min_by_key(|&t| {
        points.iter().map(move |&(x, y, dx, dy)| {
            let x = x + dx * t;
            let y = y + dy * t;
            (x * x + y * y).sqrt()
        }).sum::<f64>() as u16
    }).unwrap();

    let mut left = f64::INFINITY;
    let mut right = -f64::INFINITY;
    let mut top = f64::INFINITY;
    let mut bottom = -f64::INFINITY;
    let points: Vec<_> = points.into_iter().map(|(x, y, dx, dy)| {
        let x = x + dx * t;
        let y = y + dy * t;
        if x < left {
            left = x;
        }
        if x > right {
            right = x;
        }
        if y < top {
            top = y;
        }
        if y > bottom {
            bottom = y;
        }
        (x, y)
    }).collect();
    let mut map = vec![vec![false; (right - left) as usize + 1]; (bottom - top) as usize + 1];
    for (x, y) in points {
        let x = (x - left).round() as usize;
        let y = (y - top).round() as usize;
        map[y][x] = true;
    }

    for line in map {
        println!("{}", line.into_iter().map(|tile| if tile { '#' } else { ' ' }).collect::<String>());
    }
}
