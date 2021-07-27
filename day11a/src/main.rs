fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let grid_sn: i32 = input.trim().parse().unwrap();

    const SIZE: usize = 300;

    let map: Vec<Vec<_>> = (1..=SIZE as i32).map(|x| (1..=SIZE as i32).map(move |y| {
        let rack_id = x + 10;
        let power_level = (rack_id * y + grid_sn) * rack_id;
        power_level / 100 % 10 - 5
    }).collect()).collect();

    let (x, y) = (1..=SIZE - 2).flat_map(|x| (1..=SIZE - 2).map(move |y| (x, y))).max_by_key(|&(x, y)| {
        (0..3).map(|dx| (0..3).map(|dy| map[x - 1 + dx][y - 1 + dy]).sum::<i32>()).sum::<i32>()
    }).unwrap();

    println!("{},{}", x, y);
}
