use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let bots: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let pos = words.next().unwrap().split(&['<', '>'][..]).nth(1).unwrap()
                       .split(',').map(|coord| coord.parse::<i32>().unwrap());
        let pos: Vec<_> = pos.collect();
        let radius = words.next().unwrap().split('=').nth(1).unwrap().parse::<i32>().unwrap();
        (pos, radius)
    }).collect();

    let (biggest_pos, biggest_radius) = bots.iter().max_by_key(|&(_, radius)| radius).unwrap();
    println!("{}", bots.iter().filter(|(pos, _)| {
        biggest_pos.iter().zip(pos).map(|(biggest_pos, pos)| (biggest_pos - pos).abs()).sum::<i32>() <= *biggest_radius
    }).count());
}
