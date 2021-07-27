fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let grid_sn: i32 = input.trim().parse().unwrap();

    const SIZE: usize = 300;

    let mut sums = vec![vec![0; SIZE + 1]; SIZE + 1];
    for x in 1..=SIZE {
        for y in 1..=SIZE {
            let rack_id = x as i32 + 10;
            let power_level = (rack_id * y as i32 + grid_sn) * rack_id;
            sums[x][y] = sums[x - 1][y] + sums[x][y - 1] - sums[x - 1][y - 1] + (power_level / 100 % 10 - 5);
        }
    }

    let (x, y, size) = (1..=SIZE).flat_map(|size| {
        (1..=SIZE + 1 - size).flat_map(move |x| (1..=SIZE + 1 - size).map(move |y| (x, y, size)))
    }).max_by_key(|&(x, y, size)| {
        sums[x + size - 1][y + size - 1] - sums[x - 1][y + size - 1] - sums[x + size - 1][y - 1] + sums[x - 1][y - 1]
    }).unwrap();

    println!("{},{},{}", x, y, size);
}
