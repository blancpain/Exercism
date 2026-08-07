pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for x in 0..=n {
        sum += x;
    }

    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for x in 0..=n {
        sum += x * x;
    }

    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
