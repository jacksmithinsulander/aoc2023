fn num_parse(input: &str) -> Result<i32, std::num::ParseIntError>  {
       
    // Empty arr to fill with number values
    let mut numeric_chars: Vec<char> = Vec::new();

    // Funnel out the numbers
    for tmp_char in input.chars() {
        if tmp_char.is_numeric() {
            numeric_chars.push(tmp_char);
        }
    }

    let first_num = numeric_chars[0];

    let last_num = numeric_chars[numeric_chars.len() - 1];

    let result_str: String = format!("{}{}", first_num, last_num);

    result_str.parse::<i32>()
} 

fn main() {
    let input = include_str!("./input1.txt");

    let mut tmp_int: i32 = 0;

    // Split text by lines to iterate over
    for line in input.lines() {
        match num_parse(line) {
            Ok(num) => tmp_int += num,
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Result: {}", tmp_int);
}