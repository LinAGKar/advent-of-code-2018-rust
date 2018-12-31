use std::io;
use std::io::Read;

fn parse_node<T>(numbers: &mut T) -> Option<u32> where T:Iterator<Item = u32> {
    let child_count = numbers.next()?;
    let metadata_count = numbers.next()?;
    let mut total = 0;
    for _ in 0..child_count {
        total += parse_node(numbers)?;
    }
    for _ in 0..metadata_count {
        total += numbers.next()?;
    }
    Some(total)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    println!("{}", parse_node(&mut numbers).unwrap());
}
