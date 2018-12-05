use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("{}", input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum::<i32>());
    Ok(())
}
