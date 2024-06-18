use std::io;
use a::b;
pub mod a;

fn main() {
    print_fib();
    play_fahrenheit_game();
    print_rectangle_area();
    b::b();
}

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32f64) * (5f64 / 9f64)
}

fn play_fahrenheit_game() {
    println!("Please input degrees in Fahrenheit to convert.");

    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    let inp = inp.trim();

    if inp == "quit" || inp == "exit" {
        return;
    }

    let inp: f64 = match inp.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number or type 'quit' or 'exit' to stop.");
            return;
        }
    };

    let conversion = fahrenheit_to_celsius(inp);

    println!("{} degrees Fahrenheit is {} in Celsius.", inp, conversion)
}

fn print_rectangle_area() {
    println!("---Rectangles---");
    let r = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{} is the rectangle's area", r.area());
    
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("r can contain r2: {}", r.can_hold(&r2));
}

fn fibonacci(n: i32) -> i32 {
    let mut prev = 1;
    let mut curr = 1;
    if n == 0 {
        return 0;
    }

    for _ in 2..n {
        let temp = curr + prev;
        prev = curr;
        curr = temp;
    }

    return curr;
}

fn print_fib() {
    println!("{}", fibonacci(13));
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.height <= self.height && other.width <= self.width
    }
}
