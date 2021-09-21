use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let ip: usize = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();

    let instructions: Vec<_> = lines.map(|line| {
        let mut words = line.split_whitespace();
        let instruction = words.next().unwrap();
        let args: Vec<_> = words.map(|word| word.parse::<usize>().unwrap()).collect();
        (instruction, args[0], args[1], args[2])
    }).collect();
    let mut regs = vec![0; 6];
    regs[0] = 1;

    while let Some(&(instruction, a, b, c)) = instructions.get(regs[ip]) {
        match instruction {
            "addr" => { regs[c] = regs[a] + regs[b]; }
            "addi" => { regs[c] = regs[a] + b; }
            "mulr" => { regs[c] = regs[a] * regs[b]; }
            "muli" => { regs[c] = regs[a] * b; }
            "banr" => { regs[c] = regs[a] & regs[b]; }
            "bani" => { regs[c] = regs[a] & b; }
            "borr" => { regs[c] = regs[a] | regs[b]; }
            "bori" => { regs[c] = regs[a] | b; }
            "setr" => { regs[c] = regs[a]; }
            "seti" => { regs[c] = a; }
            "gtir" => { regs[c] = if a > regs[b] { 1 } else { 0 }; }
            "gtri" => { regs[c] = if regs[a] > b { 1 } else { 0 }; }
            "gtrr" => { regs[c] = if regs[a] > regs[b] { 1 } else { 0 }; }
            "eqir" => { regs[c] = if a == regs[b] { 1 } else { 0 }; }
            "eqri" => { regs[c] = if regs[a] == b { 1 } else { 0 }; }
            "eqrr" => { regs[c] = if regs[a] == regs[b] { 1 } else { 0 }; }
            instruction => panic!("Unimplemented instuction {}", instruction),
        }
        regs[ip] += 1;
        if regs[ip] == 28 {
            println!("{}", regs[instructions[28].1]);
            break;
        }
    }
}
