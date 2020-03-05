use std::fs;

#[derive(Eq, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.lines();

    let mut state = vec![vec![0isize; 1000]; 1000];

    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        let start_coord_token: Vec<&str> = tokens[1].split(',').collect();
        let start_coord = Coordinates {
            x: start_coord_token[0].parse::<usize>().unwrap(),
            y: start_coord_token[1].parse::<usize>().unwrap(),
        };

        let end_coord_token: Vec<&str> = tokens[2].split(',').collect();
        let end_coord = Coordinates {
            x: end_coord_token[0].parse::<usize>().unwrap(),
            y: end_coord_token[1].parse::<usize>().unwrap(),
        };
        for x in start_coord.x..(end_coord.x + 1) {
            for y in start_coord.y..(end_coord.y + 1) {
                state[x][y] += match tokens[0] {
                    "on" => 1,
                    "off" => {
                        -2
                    },
                    "toggle" => {
                        2
                    }
                   
                    _ => panic!("Invalid operation found in input file!"),
                };
                if state[x][y] < 0 {
                    state[x][y] = 0;
                }
            }
        }
    }
    let mut count = 0;
    for x in 0..999 {
        for y in 0..999 {
            count += state[x][y];
        }
    }
    print!("Total on in the end {}", count);
}
