fn function(a: i32, b: i32) -> i32 {
    // if: condition needs no parentheses, braces are mandatory, condition must be bool

    if b == 0 {
        return -1;             // also valid, but idiomatic Rust uses it only for early returns
    }
    
    println!("function called with arguments: {}, {}", a, b);
    a / b      // no semicolon = this is the return value
}

fn main() {
    let result = function(50, 10);
    println!("Result: {}", result);


    let mut count=0;
    
    loop {
        count = count + 1;
        println!("Count={}", count);

        if count == 5 {
            break;
        }
    }

    println!("The result = {result}");
    /* A unique feature of Rust's loop is its ability to return a value directly to a variable.
       You pass the value you want to return right after the break statement. */


    //while loop

    let mut number = 0;
    while number < 10 {
        println!("{}", number);
        number = number + 1;
    }

    //for loop

    /*
    Iterating Over a Range:
    You can specify a range using start..end (exclusive of the end number) or start..=end (inclusive of the end number).
    */
    for x in 1..15{
        println!("{}", x);
    }


    /*
    Iterating Over a Collection:
    By default, directly iterating over a collection via for x in collection will consume (move ownership of) the collection. If you want to use the collection later, borrow it using .iter() or &collection.
    */
    println!("Iterating Over a Collection:");
    let array = [1,2,3,4,5,6,7,8,9,10];
    for x in &array {                     // like: for (const auto& item : v)
        println!("{}", x);
    }

    println!("{}", array[0]);

    /*
    for element in a.iter() {
        println!("The value is: {element}");
    }
    */

}

/*
Things C++ has that Rust deliberately omits: function overloading (use different names or traits), 
default arguments (use the builder pattern or Option parameters), and implicit this (you write self explicitly)
*/


/*
Loops in Rust:
Rust provides three types of loops: loop, while, and for. They let you repeatedly execute 
code blocks, control flow using conditions or iterators, and can even return values.


1. The loop Keyword (Infinite Loops)

The loop keyword tells Rust to execute a block of code over and over again indefinitely until you 
explicitly tell it to stop using the break keyword. It is preferred over while true because the compiler 
knows it will loop forever, which aids in type safety and code analysis.


2. The while Loop (Conditional Loops)

while loop evaluates a condition before running each iteration. The loop executes as long as the condition remains true.


3. The for Loop (Iterating Over Collections)

The for loop is the most common and safest loop in Rust. It is used to iterate over elements of a collection (like an array or vector) or over a specific range of values.


Loop Control: break, continue, and Labels:
• break: Immediately exits the current loop.
• continue: Skips the remaining code in the current iteration and jumps directly to the next iteration


Loop Labels:
When working with nested loops, break and continue apply to the innermost loop by default. 
You can specify a loop label (which starts with a single quote ') to control an outer loop instead.

*/