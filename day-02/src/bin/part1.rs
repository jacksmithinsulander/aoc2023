fn game_parse(input: &str) -> Result<i32, std::num::ParseIntError> {
    // Setting max values
    let red_max: i32 = 12;
    let green_max: i32 = 13;
    let blue_max: i32 = 14;

    let mut is_possible: bool = true;

    // Splitting up string input by certain characters
    let split_input: Vec<&str> = input.split(|c| c == ':' || c == ';' || c == ',').collect();

    let mut tmp_id: i32 = 0;

    // Operation to se if the requirements are met
    for item in split_input {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts[0] == "Game" {
            tmp_id = parts[1].parse()?;
        } else {
            let string = parts[1];
            let number = parts[0].parse::<i32>()?;
            if (string == "blue" && number > blue_max)
                || (string == "green" && number > green_max)
                || (string == "red" && number > red_max)
            {
                is_possible = false;
            }
        }
    }

    if is_possible {
        Ok(tmp_id)
    } else {
        Ok(0)
    } 
}

fn main() {
    let input = include_str!("./input1.txt");

    let mut tmp_int: i32 = 0;


    // Split text by lines to iterate over
    for line in input.lines() {
        match game_parse(line) {
            Ok(num) => tmp_int += num,
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Result: {}", tmp_int);
}