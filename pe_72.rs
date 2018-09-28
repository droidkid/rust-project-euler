struct NumInfo {
    num: i32,
    prime_factors: Vec<i32>
}

impl NumInfo {
    fn new(num:i32) -> NumInfo {
        NumInfo {
            num: num,
            prime_factors: Vec::new(),
        }
    }
}

fn main() {

    let mut num_infos : Vec<NumInfo> = Vec::new();
    let lim = 1_000_000usize;
    for i in 0..=lim {
        num_infos.push(NumInfo::new(i as i32));
    }

    for i in 2usize..=lim {
        if num_infos[i].prime_factors.len() == 0 {
            num_infos[i].prime_factors.push(i as i32);
            let mut j = i*i;
            while j < lim {
                num_infos[i].prime_factors.push(i as i32);
                j = j + i;
            }

        }
    }

}
