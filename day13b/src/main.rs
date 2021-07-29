use std::collections::BTreeMap;
use std::io::{Read,stdin};

enum TileType {
    Track,
    Turn(i16),
    Intersection,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut carts = BTreeMap::new();

    let map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        let y = y as i16;
        line.chars().enumerate().map(|(x, chr)| {
            let x = x as i16;
            match chr {
                '+' => TileType::Intersection,
                '\\' => TileType::Turn(1),
                '/' => TileType::Turn(-1),
                'v' => {
                    carts.insert((y, x), (1, 0, -1));
                    TileType::Track
                }
                '^' => {
                    carts.insert((y, x), (-1, 0, -1));
                    TileType::Track
                }
                '<' => {
                    carts.insert((y, x), (0, -1, -1));
                    TileType::Track
                }
                '>' => {
                    carts.insert((y, x), (0, 1, -1));
                    TileType::Track
                }
                _ => TileType::Track,
            }
        }).collect()
    }).collect();

    while carts.len() > 1 {
        let mut new_carts = BTreeMap::new();

        while let Some(&(y, x)) = carts.keys().next() {
            let (dy, dx, mut intersect_dir) = carts.remove(&(y, x)).unwrap();

            let (x, y) = (x + dx, y + dy);

            if carts.contains_key(&(y, x)) {
                carts.remove(&(y, x));
            } else if new_carts.contains_key(&(y, x)) {
                new_carts.remove(&(y, x));
            } else {
                let (dx, dy) = match map[y as usize][x as usize] {
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

                new_carts.insert((y, x), (dy, dx, intersect_dir));
            }
        }

        carts = new_carts;
    }

    let &(y, x) = carts.keys().next().unwrap();
    println!("{},{}", x, y);
}
