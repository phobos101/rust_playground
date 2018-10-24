use std::io;

fn main() {
    println!("Welcome to Fibonacci finder!");
    println!("Enter a number 1-50:");

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("Please enter a value!");
    
    let input_number: u32 = input_number
        .trim()
        .parse()
        .expect("Please enter a number");
    
    let result: u64 = get_fibonacci_value(input_number);
    print_result(input_number, result);
}

fn get_fibonacci_value(num: u32) -> u64 {
    match num {
        0 => panic!("Cannot use 0!"),
        1 | 2 => 1,
        3 => 2,
        4 => 3,
        5 => 5,
        6 => 8,
        _ => get_fibonacci_value(num - 1) + get_fibonacci_value(num - 2)
    }
}

fn print_result(position: u32, result: u64) {
    println!("The fibonacci number at position {} is: {}", position, result);
}
