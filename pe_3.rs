fn main() {

    let mut lim: i64 = 600851475143;
    let sq = (lim as f64).sqrt() as i64;

    for i in 2..=sq {
        if lim % i == 0 {
            println!("{}",i);
            while lim % i == 0 {
                lim = lim / i;
            }
        }
    }
}
