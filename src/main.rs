use std::io;
fn main() {
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
