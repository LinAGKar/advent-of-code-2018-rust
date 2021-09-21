use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut constants = input.lines().skip(1).map(|line| line.split_whitespace());
    let val_start: u64 = constants.nth(7).unwrap().nth(1).unwrap().parse().unwrap();
    let factor: u64 = constants.nth(3).unwrap().nth(2).unwrap().parse().unwrap();
    let mut val = 0;
    let mut last_new = 0;
    let mut seen = HashSet::new();

    loop {
        let mut val2 = val | 0x10000;
        val = val_start;
        loop {
            val = (val + (val2 & 0xFF) & 0xFFFFFF) * factor & 0xFFFFFF;
            if 0x100 > val2 {
                break;
            }
            val2 >>= 8;
        }

        if seen.contains(&val) {
            println!("{}", last_new);
            break;
        } else {
            last_new = val;
            seen.insert(val);
        }
    }
}
