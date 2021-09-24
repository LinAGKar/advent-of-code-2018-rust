use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|line| line.split_whitespace().nth(1).unwrap());
    let depth: u32 = lines.next().unwrap().parse().unwrap();
    let target: Vec<usize> = lines.next().unwrap().split(',').map(|num| num.parse().unwrap()).collect();

    let mut map = vec![vec![0; target[1] + 1]; target[0] + 1];

    for x in 1..=target[0] {
        map[x][0] = (x as u32 * 16807 + depth) % 20183;
    }

    for y in 1..=target[1] {
        map[0][y] = (y as u32 * 48271 + depth) % 20183;
    }

    for x in 1..=target[0] {
        for y in 1..=target[1] {
            if (x, y) != (target[0], target[1]) {
                map[x][y] = (map[x - 1][y] * map[x][y - 1] + depth) % 20183;
            }
        }
    }

    map[target[0]][target[1]] = 0;

    println!("{}", map.into_iter().flat_map(|col| col).map(|erosion| erosion % 3).sum::<u32>());
}
