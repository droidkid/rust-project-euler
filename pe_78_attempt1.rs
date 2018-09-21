
fn solve(cache: &mut Vec<Vec<i64>>, n:i32, k:i32) -> i64 {
    if n==0 {
        return 1;
    }
    if n < 0 {
        return 0;
    }
    if n < k {
        return solve(cache, n, n);
    }

    if cache[n as usize][k as usize] != -1 {
        return cache[n as usize][k as usize];
    }


    let mut ret:i64 = 0;
    for i in 1..=k {
        ret = (ret + solve(cache, n-i, i)) % 1000000;
    }
    cache[n as usize][k as usize] = ret;
    ret
}

fn main() {
    let mut cache: Vec< Vec<i64> > = Vec::new(); 
    let mut ans: Vec<i32> = Vec::new();

    cache.push(vec![-1; 2000]);

    for i in 1..2000 {
        cache.push(vec![-1; 2000]);
        let tmp = solve(&mut cache, i, i);
        println!("{} {}", i, tmp);

        if tmp == 0 {
            ans.push(i);
        }
    }

    println!("{:?}", ans);

}
