use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}
impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Couldn't read input file :(");
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut santa_pos = Coordinates { x: 0, y: 0 };
    let mut robot_pos = Coordinates { x: 0, y: 0 };
    let mut santas_turn = true;
    for d in contents.chars() {
        visited.insert(Coordinates {
            x: santa_pos.x,
            y: santa_pos.y,
        });
        visited.insert(Coordinates {
            x: robot_pos.x,
            y: robot_pos.y,
        });
        let mover = match santas_turn {
            true => &mut santa_pos,
            false => &mut robot_pos,
        };
        match d {
            'v' => (*mover).y -= 1,
            '^' => (*mover).y += 1,
            '<' => (*mover).x -= 1,
            '>' => (*mover).x += 1,
            _ => panic!("unexpected input character"),
        };
        santas_turn = !santas_turn;
    }
    print!("Total number of visited houses: {}", visited.len());
}
