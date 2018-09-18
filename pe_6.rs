fn main () {
    let mut sum_of_squares:u64 = 0;
    let mut sum_squared:u64 = 0;

    for i in 1..=100 {
        sum_of_squares += i*i;
        sum_squared += i;
    }
    sum_squared = sum_squared*sum_squared;

    println!("{}, {}, {}", sum_squared, sum_of_squares, sum_squared-sum_of_squares);
}
