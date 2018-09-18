fn reverse_number(mut num:u32) -> u32 {
    let mut ret = 0;
    while num > 0 {
        ret = ret * 10 + num%10;
        num = num / 10;
    }
    ret
}

fn is_palindrome(num:u32) -> bool {
    if reverse_number(num) == num {
        return true;
    }
    false
}

fn main() {
    let mut ans = 0;

    for i in 100..=999 {
        for j in 100..=999 {
            if is_palindrome(i*j) && ans < i*j {
                ans = i*j;
            }
        }
    }
    println!("{}", ans);
}
