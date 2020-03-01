use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Couldn't read input file :(");
    
    let mut total_paper_used = 0;
    let mut total_ribbon_used = 0;
    for line in contents.lines() {
        let mut len = 0;
        let mut wid = 0;
        let mut hei = 0;

        for (i, token) in line.split('x').enumerate() {
            let int_value = token.parse().unwrap();
            match i {
                0 => len=int_value,
                1 => wid=int_value,
                2 => hei=int_value,
                _ => panic!("incorrect number of tokens in dimensions string {}", line)
            };
        }
        let mut smallest = len*wid;
        if wid*hei < smallest {
            smallest = wid*hei;
        }
        if hei*len < smallest {
            smallest = hei*len;
        }

        let mut highest = len;
        if highest < wid {
            highest = wid;
        }
        if highest < hei {
            highest = hei;
        }

        total_paper_used += 2*len*wid + 2*wid*hei + 2*hei*len + smallest;
        total_ribbon_used += len+len+wid+wid+hei+hei + (len*wid*hei) - (highest+highest);

    }
    print!("Total paper used: {}", total_paper_used);
    print!("Total ribbon used: {}", total_ribbon_used);
}
