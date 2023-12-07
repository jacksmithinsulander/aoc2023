fn main() {
    let input = "Time:        42     68     69     85
        Distance:   284   1005   1122   1341";

    let mut lines_iter = input.lines();

    let line_1 = lines_iter.next().unwrap();

    let line_2 = lines_iter.next().unwrap();

    let time_vec: Vec<i32> = line_1
        .split_whitespace()
        .skip(1)
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let dist_vec: Vec<i32> = line_2
        .split_whitespace()
        .skip(1)
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    println!("{:?}, {:?}", time_vec, dist_vec);
    
    let mut win_divisor: i32 = 0;



}