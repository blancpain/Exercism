pub fn is_armstrong_number(num: u32) -> bool {
    let digit_len = num.to_string().len();
    let vec_num: u32 = num
        .to_string()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| x.pow(digit_len as u32))
        .sum();

    vec_num == num
}
