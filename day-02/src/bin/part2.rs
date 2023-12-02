fn game_parse(input: &str) -> Result<i32, std::num::ParseIntError> {
    // Splitting up string input by certain characters
    let split_input: Vec<&str> = input.split(|c| c == ':' || c == ';' || c == ',').collect();

    // Setting up color arrays to store seperated color values
    let mut blue_items = vec![];
    let mut green_items = vec![];
    let mut red_items = vec![];

    // Operation to sort values into arrays
    for item in split_input {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts[0] == "Game" {
            continue;
        } else {
            let string = parts[1];
            let number = parts[0].parse::<i32>()?;
            if string == "blue"{
                blue_items.push(number);
            } else if string == "green" {
                green_items.push(number);
            } else if string == "red" {
                red_items.push(number);
            }
        }
    }

    // Finding the highest values in each array
    let blue_amount = blue_items.iter().max().unwrap_or(&0);
    let green_amount = green_items.iter().max().unwrap_or(&0);
    let red_amount = red_items.iter().max().unwrap_or(&0);

    Ok(*blue_amount * *green_amount * *red_amount)
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