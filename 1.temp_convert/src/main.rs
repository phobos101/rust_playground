use std::io;

fn main() {
    print_menu();

    let choice: String = get_input();
    let choice: f32 = parse_to_float(choice);

    let mut source_format: String = "fahrenheit".to_string();
    let mut dest_format: String = "celcius".to_string();
    if choice != 1.0 {
        source_format = "celcius".to_string();
        dest_format = "fahrenheit".to_string();
    }

    println!("Enter the value in {}", source_format);
    let temp: String = get_input();
    let temp: f32 = parse_to_float(temp);

    convert_tempreture(dest_format, temp)
}

fn print_menu() {
    println!("========================");
    println!("Please choose an option:");
    println!("1) Convert to celcius");
    println!("2) Convert to fahrenheit");
    println!("========================");
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("You must enter a string");

    return input;
}

fn parse_to_float(str: String) -> f32 {
    return str.trim().parse()
        .expect("Please enter a number");
}

fn convert_tempreture(dest_format: String, temp: f32) {
    let converted_temp = if dest_format == "celcius" {
        (temp - 32.0) * 5.0/9.0
    } else {
        (temp * 9.0/5.0) + 32.0
    };

    println!("The tempreture in {} is: {}", dest_format, converted_temp);
}