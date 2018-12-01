use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let res = io::stdin().read_to_string(&mut buffer);
    match res {
        Err(_) => {
            println!("error reading stdin!");
            panic!();
        }
        _ => {}
    }

    let freqs: Vec<&str> = buffer.split("\n").map(|e| e.trim()).collect();
    let mut saved_freqs: HashSet<i32> = HashSet::new();
    let mut found = false;
    let mut first = true;

    let mut out = 0;
    while !found {
        for freq in &freqs {
            if let Ok(value) = freq.parse::<i32>() {
                out += value;

                if saved_freqs.contains(&out) {
                    println!("first freq reoccurrence: {}", out);
                    found = true;
                    break;
                } else {
                    saved_freqs.insert(out);
                }
            }
        }
        
        if first {
            println!("freq change: {}", out);
        }

        first = false;
    }
}
