fn line_parse(input: &str) -> i32 {
    // Split line by divisor sign
    let parts: Vec<&str> = input.split('|').map(|s| s.trim()).collect();

    //let [first_part, second_part] = parts;

    let mut tmp_ret: i32 = 0;

    let first_part = parts[0];
    let second_part = parts[1];

    let a: Vec<i32> = first_part
        .split_whitespace()
        .skip(2)
        .map(|s| s.parse().unwrap())
        .collect();
    let b: Vec<i32> = second_part
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut tmp_sum = 0;

    for a_num in &a {
        if b.contains(a_num) {
            tmp_sum += 1;
        }
    }

    if tmp_sum == 0 || tmp_sum == 1 {
        tmp_ret = tmp_sum
    } else {
        tmp_ret = 2_i32.pow((tmp_sum - 1) as u32)
    }

    println!("tmp_sum: {}", tmp_ret);
    return tmp_ret
}

fn main() {
    let input = include_str!("./input1.txt");

    let mut tmp_int: i32 = 0;

    // Split text by lines to iterate over
    for line in input.lines() {
        tmp_int += line_parse(line);
    }

    println!("Result: {}", tmp_int);
}