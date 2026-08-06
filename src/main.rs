use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed"); 
    
    let number: i32 = input
        .trim()
        .parse()
        .expect("not a number");
    
    println!("{}", number);
    println!("{}", parse_string_add_one(input.as_str().trim()));
}

fn parse_string_add_one(num: &str) -> i32 {
    let number: Result<i32, std::num::ParseIntError> = num.parse::<i32>();
    match number {
        Ok(_) => {return number.unwrap_or_default()},
        Err(_) => {
            dbg!(number.unwrap_err());
        },
    }
    return 0;
}
