fn main() {
    let secret = "iwrupvqb";
    let mut attempt = String::new();
    let mut counter = 0;
    while !attempt.starts_with("000000") {
        let digest = md5::compute(&(format!("{}{}", &secret, &counter)));
        attempt = format!("{:x}", digest);
        counter += 1;
    }
    print!("The winning number is: {}", counter - 1);
}
