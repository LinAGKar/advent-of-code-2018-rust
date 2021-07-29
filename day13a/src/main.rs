use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{Read,stdin};

enum TileType {
    Track,
    Turn(i16),
    Intersection,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut carts = BinaryHeap::new();

    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        let y = y as i16;
        line.chars().enumerate().map(|(x, chr)| {
            let x = x as i16;
            match chr {
                '+' => (false, TileType::Intersection),
                '\\' => (false, TileType::Turn(1)),
                '/' => (false, TileType::Turn(-1)),
                'v' => {
                    carts.push((Reverse(y), Reverse(x), y, x, 1, 0, -1));
                    (true, TileType::Track)
                }
                '^' => {
                    carts.push((Reverse(y), Reverse(x), y, x, -1, 0, -1));
                    (true, TileType::Track)
                }
                '<' => {
                    carts.push((Reverse(y), Reverse(x), y, x, 0, -1, -1));
                    (true, TileType::Track)
                }
                '>' => {
                    carts.push((Reverse(y), Reverse(x), y, x, 0, 1, -1));
                    (true, TileType::Track)
                }
                _ => (false, TileType::Track),
            }
        }).collect()
    }).collect();

    'outer: loop {
        let mut new_carts = BinaryHeap::new();

        for (_, _, y, x, dy, dx, mut intersect_dir) in carts {
            map[y as usize][x as usize].0 = false;

            let (x, y) = (x + dx, y + dy);

            let tile = &mut map[y as usize][x as usize];
            if tile.0 {
                println!("{},{}", x, y);
                break 'outer;
            }
            tile.0 = true;

            let (dx, dy) = match tile.1 {
                TileType::Track => (dx, dy),
                TileType::Turn(direction) => (dy * direction, dx * direction),
                TileType::Intersection => {
                    intersect_dir = (intersect_dir + 1) % 3;
                    match intersect_dir {
                        0 => (dy, -dx),
                        1 => (dx, dy),
                        2 => (-dy, dx),
                        _ => panic!(""),
                    }
                }
            };

            new_carts.push((Reverse(y), Reverse(x), y, x, dy, dx, intersect_dir));
        }

        carts = new_carts;
    }
}
