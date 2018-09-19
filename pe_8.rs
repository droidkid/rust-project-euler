use std::io;
use std::cmp;

fn calc(line: &[u8], seq_len:usize) -> u64 {
    let mut ret:u64 = 1;
    let zero = '0' as u64;
    for i in 0..(line.len()-seq_len) {
        let mut tmp:u64 = 1;
        for j in 0..seq_len {
            let val = line[i+j] as u64;
            tmp = tmp * (val - zero);
        }
        ret = cmp::max(tmp, ret);
    }
    ret
}

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(error) => {
            println!("{}", error);
        }
    };

    let ans = calc(input.as_bytes(), 13);

    println!("{}", ans);
}
