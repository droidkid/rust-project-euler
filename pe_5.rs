fn is_prime(num:i32)->bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
fn main() {
    
    // For every prime number, multiply it to it's max power.
    let mut ans:i64 = 1;

    for i in 2..=20 {
        if is_prime(i) {
            let mut a:i64 = 1;
            loop {
                a = a * (i as i64);
                if a > 20 {
                    break;
                }
                ans = ans * (i as i64);
            }
        }
    }

    println!("{}", ans);

}
