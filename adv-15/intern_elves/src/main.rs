use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.lines();

    let mut nice_words = 0;
    for line in lines {
        let mut previous_char = line.chars().nth(0).unwrap();
        let mut second_previous_char = ' ';
        let mut map = HashMap::<String, usize>::new();
        let mut has_repeated_char = false;
        let mut has_repeated_arr = false;
        for i in 1..line.len() {
            let ch = line.chars().nth(i).unwrap();

            if !has_repeated_char && i > 1 {
                if second_previous_char == ch {
                    has_repeated_char = true;
                }
            }
            let last_two = &format!("{}{}", previous_char, ch);
            match map.get(last_two) {
                Some(x) => {
                    if i > (x + 1) {
                        has_repeated_arr = true;
                    }
                }
                None => {
                    map.insert(format!("{}{}", previous_char, ch), i);
                }
            }

            second_previous_char = previous_char;
            previous_char = ch;
        }
        if has_repeated_char && has_repeated_arr {
            nice_words += 1;
        }
    }
    print!("Number of nice words in input: {}", nice_words);
}
