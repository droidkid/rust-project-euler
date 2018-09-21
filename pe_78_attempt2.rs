
fn penta(k: i64) -> i64 {
    if k%2 == 0 {
        let n = k/2;
        return (n * ( 3 * n + 1 )) / 2;
    }
    else {
        let n = (k+1)/ 2;
        return (n * (3 * n - 1)) / 2;
    }
}

fn main () {
    let mut cache: Vec<i32> = Vec::new();
    let mut ans: Vec<i64> = Vec::new();

    cache.push(1);
    cache.push(1);

    for i in 2..500000 {
        let mut sign = 1;
        let mut a = 1;
        let mut sum = 0;

        while penta(a) <= i {
            let idx:usize = (i-penta(a)) as usize;
            sum = (sum + sign * cache[idx]) % 1_000_000;
            if a % 2 == 0 {
                sign = sign * -1;
            }
            a = a + 1;
        }
        if sum == 0 {
            ans.push(i);
        }
        if sum < 0 {
            sum = sum + 1_000_000;
        }
        cache.push(sum);
        if i % 1000 == 0 {
            println!("{} {}", i, sum);
        }
    }

    println!("{:?}", ans);

    println!("{:?}", -1 % 1_000_000);

}
