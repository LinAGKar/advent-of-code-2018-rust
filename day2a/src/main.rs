use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (doubles, triples) = input.split_whitespace().map(|id| {
        let mut letter_occurences = HashMap::new();
        for letter in id.chars() {
            if !letter_occurences.contains_key(&letter) {
                letter_occurences.insert(letter, 0);
            }
            *letter_occurences.get_mut(&letter).unwrap() += 1;
        }
        let mut has_double = 0;
        let mut has_triple = 0;
        for &occurences in letter_occurences.values() {
            if occurences == 2 {
                has_double = 1;
            }
            if occurences == 3 {
                has_triple = 1;
            }
            if has_double == 1 && has_triple == 1 {
                break;
            }
        }
        (has_double, has_triple)
    }).fold((0, 0), |(acc_double, acc_triple), (has_double, has_triple)| (acc_double + has_double, acc_triple + has_triple));
    println!("{}", doubles * triples);
    Ok(())
}
