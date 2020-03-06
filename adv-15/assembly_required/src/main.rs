use std::collections::HashMap;
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
    let mut map = HashMap::<String, u16>::new();

    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.lines();
    let mut operations: Vec<Operation> = Vec::new();
    for line in lines {
        let tokens:Vec<&str> = line.split(' ').collect();
        let operation = create_operation(tokens);
        operations.push(operation);

    }
    let mut executed_operation = true;
    while executed_operation {
        executed_operation = false;
        for operation in &mut operations {
        perform_operation(operation, &mut map);
        }
    }
}

fn create_operation(tokens: Vec<&str>) -> Operation {
    match tokens.len() {
        3=> Operation::Assign(create_value(tokens[0].to_string()), create_value(tokens[2].to_string())),
        4=> Operation::Not(create_value(tokens[1].to_string()), create_value(tokens[3].to_string())),
        5=> {
            match tokens[1] {
                "AND" => Operation::And(create_value(tokens[0].to_string()), create_value(tokens[2].to_string()), create_value(tokens[4].to_string())),
                "OR" => Operation::Or(create_value(tokens[0].to_string()), create_value(tokens[2].to_string()), create_value(tokens[4].to_string())),
                "LShift" => Operation::LShift(create_value(tokens[0].to_string()), create_value(tokens[2].to_string()), create_value(tokens[4].to_string())),
                "RShift" => Operation::RShift(create_value(tokens[0].to_string()), create_value(tokens[2].to_string()), create_value(tokens[4].to_string())),
                _ => panic!("Unsupported operation! {}", tokens[1])
            }
        },
        _=> panic!("Unsupported token length! {}", tokens.len())
    }
}



fn create_value(value : String) -> Value {
    if value.chars().nth(0).unwrap().is_numeric() {
        return Value::Literal(value.parse::<u16>().unwrap());
    }
    return Value::Reference(value);
}

fn get_value(value: Value, map: &HashMap::<String, u16>) -> Option<u16> {
    match value {
        Value::Literal(l) => Some(l),
        Value::Reference(r) => {
           match map.get(&r) {
               Some(i) => Some(*i),
               None => None
           }
        }
    }
}
fn get_reference(value: Value) -> String {
    match value {
        Value::Reference(r) => r,
        _=> panic!("Cannot get reference of literal value!")
    }
}

fn perform_operation(operation: &mut Operation, map: &mut HashMap::<String, u16>) {
    let target:Value;
    let result = match operation {
        Operation::Assign(v1,t) => {
            target = t;
            get_value(v1, &map).unwrap()
        }
        Operation::And(v1,v2,t) => {
            target = t;
            get_value(v1, &map).unwrap() & get_value(v2, &map).unwrap()
        },
        Operation::Or(v1,v2,t) => {
            target = t;
            get_value(v1, &map).unwrap() | get_value(v2, &map).unwrap()
        },
        Operation::Not(v1,t) => {
            target = t;
            !get_value(v1, &map).unwrap()
        },
        Operation::LShift(v1,v2,t) => {
            target = t;
            get_value(v1,&map).unwrap() << get_value(v2, &map).unwrap()
        },
        Operation::RShift(v1,v2,t) => {
            target = t;
            get_value(v1,&map).unwrap() >> get_value(v2, &map).unwrap()
        },
        _ => panic!("unsupported operation!")
    };
    let value = map.entry(get_reference(target)).or_insert(0);
    *value = result;
    println!("we did it! {:?}", map);
}
