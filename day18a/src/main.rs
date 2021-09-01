use std::io::Read;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|tile| match tile {
        '.' => Tile::Open,
        '|' => Tile::Trees,
        '#' => Tile::Lumberyard,
        _ => panic!(),
    }).collect()).collect();

    let width = map[0].len();
    let height = map.len();

    for row in &mut map {
        row.insert(0, Tile::Open);
        row.push(Tile::Open);
    }

    map.insert(0, vec![Tile::Open; width + 2]);
    map.push(vec![Tile::Open; width + 2]);

    let mut new_map = map.clone();

    let neighboors: Vec<_> = (0..3).flat_map(|dy| (0..3).map(move |dx| (dy, dx))).filter(|&d| d != (1, 1)).collect();
    const DURATION: i32 = 10;

    for _ in 0..DURATION {
        for y in 1..height + 1 {
            for x in 1..width + 1 {
                new_map[y][x] = match map[y][x] {
                    Tile::Open => if neighboors.iter().filter(|&(dy, dx)| {
                        map[y - 1 + dy][x - 1 + dx] == Tile::Trees
                    }).count() >= 3 {
                        Tile::Trees
                    } else {
                        Tile::Open
                    }

                    Tile::Trees => if neighboors.iter().filter(|&(dy, dx)| {
                        map[y - 1 + dy][x - 1 + dx] == Tile::Lumberyard
                    }).count() >= 3 {
                        Tile::Lumberyard
                    } else {
                        Tile::Trees
                    }

                    Tile::Lumberyard => if neighboors.iter().any(|(dy, dx)| {
                        map[y - 1 + dy][x - 1 + dx] == Tile::Lumberyard
                    }) && neighboors.iter().any(|(dy, dx)| {
                        map[y - 1 + dy][x - 1 + dx] == Tile::Trees
                    }) {
                        Tile::Lumberyard
                    } else {
                        Tile::Open
                    }
                }
            }
        }
        std::mem::swap(&mut map, &mut new_map);
    }

    println!(
        "{}",
        map.iter().flat_map(|row| row).filter(|&&tile| tile == Tile::Trees).count() *
        map.iter().flat_map(|row| row).filter(|&&tile| tile == Tile::Lumberyard).count(),
    )
}
