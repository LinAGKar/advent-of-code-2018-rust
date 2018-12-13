use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut previous_ids = HashSet::<&str>::new();
    for i in input.split_whitespace() {
        'previous_id_loop: for j in &previous_ids {
            let mut differ = false;
            let mut differing_pos = 0;
            for (n, (k, l)) in i.chars().zip(j.chars()).enumerate() {
                if k != l {
                    differing_pos = n;
                    if differ {
                        continue 'previous_id_loop;
                    } else {
                        differ = true;
                    }
                }
            }
            for (n, k) in i.chars().enumerate() {
                if n != differing_pos {
                    print!("{}", k);
                }
            }
            println!("");
            return Ok(());
        }
        previous_ids.insert(i);
    }
    Ok(())
}
