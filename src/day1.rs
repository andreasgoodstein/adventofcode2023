const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eighth", "nine",
];

pub fn solve1(input: Vec<String>) {
    let mut sum: usize = 0;

    for line in input {
        let first_number_string = parse_number_string(line.chars().collect());
        let last_number_string = parse_number_string(line.chars().rev().collect());

        let number_string = format!("{}{}", first_number_string, last_number_string);

        sum += number_string.parse::<usize>().unwrap()
    }

    println!("{sum}");
}

pub fn solve2(input: Vec<String>) {
    let mut sum: usize = 0;

    for line in input {
        let replaced_line = replace_first_written_number(&line, false);
        let reversed_replaced_line = replace_first_written_number(&line, true);

        let first_number_string = parse_number_string(replaced_line.chars().collect());
        let last_number_string =
            parse_number_string(reversed_replaced_line.chars().rev().collect());

        let number_string = format!("{}{}", first_number_string, last_number_string);

        sum += number_string.parse::<usize>().unwrap()
    }

    println!("{sum}");
}

fn parse_number_string(char_list: Vec<char>) -> String {
    for char in char_list {
        if char.is_ascii_digit() {
            return char.to_string();
        }
    }

    "".to_string()
}

fn replace_first_written_number(line: &String, reverse: bool) -> String {
    let digit_at_index = DIGITS.map(|digit| {
        if reverse {
            let reverse_digit: String = digit.chars().rev().collect();

            line.chars().rev().collect::<String>().find(&reverse_digit)
        } else {
            line.find(digit)
        }
    });

    let mut lowest_digit = usize::MAX;
    let mut lowest_index = usize::MAX;
    for (index, digit) in digit_at_index.iter().enumerate() {
        if digit.unwrap_or(usize::MAX) < lowest_digit {
            lowest_index = index;
            lowest_digit = digit.unwrap();
        }
    }

    if lowest_digit == usize::MAX {
        line.to_string()
    } else {
        let replacing_digit_string = DIGITS.get(lowest_index).unwrap();
        let replacing_digit = lowest_index + 1;

        line.replace(replacing_digit_string, &replacing_digit.to_string())
    }
}
