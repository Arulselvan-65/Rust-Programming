use std::io;

fn main() {
    let mut x = String::new();
    let mut _value = String::new();

    println!("1. Fahrenheit to Celsius.\n2. Celsius to Fahrenheit.");

    println!("Enter your choice : ");
    io::stdin().read_line(&mut x).expect("Failed to read input");
    let choice: i8 = x.trim().parse().expect("Invalid Number Format!!");

    println!("Enter the value: ");
    io::stdin().read_line(&mut _value).expect("Failed to read input");
    let value: f32 = _value.trim().parse().expect("Invalid Number!!");

    if choice == 1 {
        let celsius: f32 = (value - 32.0) * 5.0/9.0;
        println!("Fahrenheit : {}", value);
        print!("Celsius : {}", celsius);
    }

    else if choice == 2 {
        let fahrenheit: f32 = value * 9.0/5.0 + 32.0;
        println!("Celsius : {}", value);
        print!("Fahrenheit : {}", fahrenheit);
    }

    else{
        print!("Invalid choice!!");
    }

}