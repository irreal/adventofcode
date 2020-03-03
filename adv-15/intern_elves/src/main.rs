use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.lines();

    let mut nice_words = 0;
    for line in lines {
        let mut vowel_count = [0, 0, 0, 0, 0]; // a e i o u
        let mut contains_double = false;
        let mut contains_bad = false;
        let mut previous_char = ' ';
        for i in 0..line.len() {
            let ch = line.chars().nth(i).unwrap();

            match ch {
                'a' => vowel_count[0] += 1,
                'e' => vowel_count[1] += 1,
                'i' => vowel_count[2] += 1,
                'o' => vowel_count[3] += 1,
                'u' => vowel_count[4] += 1,
                _ => (),
            };

            if i > 0 {
                match format!("{}{}", previous_char, ch).as_str() {
                    "ab" | "cd" | "pq" | "xy" => {
                        contains_bad = true;
                        break;
                    }
                    _ => (),
                };
                if previous_char == ch {
                    contains_double = true;
                }
            }

            previous_char = ch;
        }
        if !contains_bad && vowel_count.iter().sum::<u32>() >= 3 && contains_double {
            nice_words += 1;
        }
    }
    print!("Number of nice words in input: {}", nice_words);
}
