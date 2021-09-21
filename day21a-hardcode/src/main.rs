use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut constants = input.lines().skip(1).map(|line| line.split_whitespace());
    let mut val: u64 = constants.nth(7).unwrap().nth(1).unwrap().parse().unwrap();
    let mut val2: u64 = 0x10000;
    let factor: u64 = constants.nth(3).unwrap().nth(2).unwrap().parse().unwrap();
    while val2 > 0 {
        val = (val + (val2 & 0xFF) & 0xFFFFFF) * factor & 0xFFFFFF;
        val2 >>= 8;
    }
    println!("{}", val);
}
