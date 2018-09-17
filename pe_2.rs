fn main() {
    let lim = 4_000_000;
    let mut f_a = 1;
    let mut f_b = 2;
    let mut sum: u64 = 0;

    while f_b < lim {
        sum = 
            if f_b % 2 == 0 {
                sum + f_b
            } else {
                sum
            };
            
        let tmp = f_b;
        f_b = f_a + f_b;
        f_a = tmp;
    }

    println!("{}", sum);
}
