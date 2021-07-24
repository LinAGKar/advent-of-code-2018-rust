fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut words = input.split_whitespace();
    let player_count: usize = words.next().unwrap().parse().unwrap();
    let end_value = words.nth(5).unwrap().parse::<u32>().unwrap();

    let mut marbles = vec![(0u32, 0u32); end_value as usize + 1];
    let mut points = vec![0; player_count];
    let mut active = 0;

    for (value, player) in (1..=end_value).zip((0..player_count).cycle()) {
        if value % 23 == 0 {
            points[player] += value;
            for _ in 0..6 {
                active = marbles[active as usize].0;
            }
            points[player] += marbles[active as usize].0;
            let before = (0..2).fold(active, |acc, _| marbles[acc as usize].0);
            marbles[active as usize].0 = before;
            marbles[before as usize].1 = active;
        } else {
            let before = marbles[active as usize].1;
            let after = marbles[before as usize].1;
            marbles[before as usize].1 = value;
            marbles[after as usize].0 = value;
            marbles[value as usize].0 = before;
            marbles[value as usize].1 = after;
            active = value;
        }
    }

    println!("{}", points.into_iter().max().unwrap());
}
