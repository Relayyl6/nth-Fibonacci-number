use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    }

    let mut prev: u64 = 0;
    let mut curr: u64 = 1;

    for _ in 2..=n {
        let next: u64 = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn main() {
    // Prints prompt to the console, requesting input
    println!{"Enter the value of n to get the nth Fibonacci number:"};

    // String::new() creates a new empty String object, that will store the user's input
    let mut input: String = String::new(); 

     // acccesses the standard input stream (the console input)
    io::stdin()       

    // .readline, // reads a line of input from the user and appends it to the input string
    //  &mut input, passes mutable reference to the input varaible and allows read_line to modify it 
        .read_line(&mut input)
        // This handles potential errors
        .expect("Failed to nread line");

        //This converts the user's input, a string, into an unsigned 32-bit integer
    // .trim() removes any leading or trailing whitespace from the input string
    // .parse() converts the trimmed string into a number, specifically u32 integer
    // .expect() handles errors
    let n: u32 = input.trim().parse().expect("Please Enter a valid number");
    let result: u64 = fibonacci(n);


   

    println!("The {n}th Fibonacci number is {result}");

}
