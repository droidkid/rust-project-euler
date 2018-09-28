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

fn calc_factors_for_combo(num_info: &NumInfo, bitmask: i32) -> i32{
    let mut combo_value = -1;
    for i in 0..num_info.prime_factors.len() {
        if (bitmask & (1 << i)) != 0 {
            combo_value = -1 * combo_value * num_info.prime_factors[i];
        }
    }

    (num_info.num - 1) / combo_value
}

fn calc_common_div(num_info:&NumInfo) -> i32 {
    let mut result = 0;
    let total_combinations = 1 << (num_info.prime_factors.len());

    for bitmask in 1..total_combinations {
        result = result + calc_factors_for_combo(&num_info, bitmask);
    }
    result
}

fn main() {

    let mut num_infos : Vec<NumInfo> = Vec::new();
    let lim = 8usize;
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

    let mut result: i64 = 0;
    for i in 2usize..=lim {
        result = result + (calc_common_div(&num_infos[i]) as i64);
    }

    println!("{}", result);

}
