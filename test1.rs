// --- Constants ---
const VERSION: &str = "0.1.0"; // This is a string constant
const MAX_VALUE: i32 = 100;    // Maximum value for testing

// --- Functions ---
fn add(a: i32, b: i32) -> i32 {
    // Add two numbers and return the result
    a + b
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1; // Base case
    }
    n * factorial(n - 1) // Recursive call
}

// --- Struct and Enum ---
struct Point {
    x: f64,
    y: f64,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// --- Main Function ---
fn main() {
    println!("Welcome to the editor!"); // Test for string highlighting

    let point = Point { x: 1.0, y: 2.0 }; // Struct instantiation
    let direction = Direction::Up;       // Enum usage

    // Call some functions
    let sum = add(10, 20);
    println!("The sum is: {}", sum);

    let fact = factorial(5);
    println!("Factorial of 5 is: {}", fact);

    // Loop example
    for i in 0..MAX_VALUE {
        if i % 10 == 0 {
            println!("i is now: {}", i); // Highlight numbers and strings
        }
    }

    // --- Multi-line Comment ---
    /*
        This is a multi-line comment.
        It should be highlighted properly.
    */

    // --- Unused Variables (to check keyword highlighting) ---
    let _unused = 42; // Test for unused variable
}
