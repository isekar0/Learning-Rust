use std::io;

fn main() {
    let mut user_fibonacci_index = String::new();

    println!("Which Fibonacci number would you like to see (starting from 0th fibonacci number: ");

    io::stdin()
        .read_line(&mut user_fibonacci_index)
        .expect("Failed to read line!");

    let user_fibonacci_index: u32 = match user_fibonacci_index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number!");
            return;
        }
    };

    println!(
        "Fibonacci num. {user_fibonacci_index} is {}",
        nth_fib(user_fibonacci_index)
    );
}

fn nth_fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    nth_fib(n - 1) + nth_fib(n - 2)
}
