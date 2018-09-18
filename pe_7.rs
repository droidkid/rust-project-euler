fn sieve(a: &mut [i32], n: usize, nth: u32) {

    for i in 0..n {
        a[i as usize] = 0;
    }

    a[0] = 1;
    a[1] = 1;

    let mut p_count = 0;

    for i in 2..n {
        let p = i as usize;
        if a[p] == 0 {
            // p is prime.
            p_count = p_count + 1;

            if p_count == nth {
                println!("{}: {}", p_count, p);
                return;
            }

            let mut c:usize = p * p;
            while c < n {
                a[c] = 1;
                c = c + p;
            }
        }
    }


}

fn main () {
    let mut a: [i32; 1_000_000] = [0; 1_000_000];
    sieve(&mut a, 1_000_000, 10001);
}
