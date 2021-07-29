fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let delay: usize = input.trim().parse().unwrap();

    let mut recipes = vec![3, 7];
    let mut elves = vec![0, 1];

    while recipes.len() < delay + 10 {
        let new: u8 = elves.iter().map(|&elf| recipes[elf]).sum();
        if new >= 10 {
            recipes.push(1);
            recipes.push(new - 10);
        } else {
            recipes.push(new);
        }
        for i in &mut elves {
            *i = (*i + recipes[*i] as usize + 1) % recipes.len();
        }
    }

    println!(
        "{}",
        recipes.into_iter().skip(delay).take(10).map(|i| char::from_digit(i as u32, 10).unwrap()).collect::<String>(),
    );
}
