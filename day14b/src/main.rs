fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let target: Vec<_> = input.trim().chars().map(|chr| chr.to_digit(10).unwrap() as u8).collect();

    let mut recipes = vec![3, 7];
    let mut elves = vec![0, 1];
    let mut pos = 0;

    'outer: loop {
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

        if pos + target.len() <= recipes.len() {
            for i in pos..recipes.len() - target.len() {
                if recipes[i..i + target.len()] == target {
                    println!("{}", i);
                    break 'outer;
                }
            }
            pos = recipes.len() - target.len();
        }
    }
}
