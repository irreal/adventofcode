use std::fs;

#[derive(Debug)]
enum Value {
    Literal(u16),
    Reference(String)
}

#[derive(Debug)]
enum Operation {
    Assign(Value, Value),
    And(Value, Value, Value),
    Or(Value,Value,Value),
    Not(Value, Value),
    LShift(Value,Value,Value),
    RShift(Value,Value,Value)
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.lines();
    for line in lines {
        let tokens:Vec<&str> = line.split(' ').collect();
        let operation = CreateOperation(tokens);
        match operation {
            Operation::Assign(v1,v2) => {print!("We got an assign! {:?} {:?}", v1, v2)},
            _=> {print!("We aint got no assign here")}
        }
    }
}

fn CreateOperation(tokens: Vec<&str>) -> Operation {
    if tokens.len() == 3 {
     return Operation::Assign(CreateValue(tokens[0].to_string()), CreateValue(tokens[2].to_string()));   
    }
    panic!("panika!");
}



fn CreateValue(value : String) -> Value {
    if value.chars().nth(0).unwrap().is_numeric() {
        return Value::Literal(16);
    }
    return Value::Reference(value);
}
