use std::io;
fn main() {
    print_fib();
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

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32f64) * (5f64 / 9f64)
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
