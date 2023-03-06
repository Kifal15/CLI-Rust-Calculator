# Rust Command Line Calculator
This is a basic calculator written in Rust that runs in the command line. It takes in two arguments and an operator as inputs and performs the requested operation.

Usage
To use the calculator, you must first compile it using the Rust package manager, Cargo. Here's how to do that:

Make sure you have Rust and Cargo installed on your machine. You can check this by running the command `rustc --version` and `cargo --version` in your terminal.

Clone this repository to your local machine.

Navigate to the project directory using the terminal.

Run the command `cargo build --release`. This will compile the calculator in release mode and create a binary file in the `target/release/` directory.

To run the calculator, use the following command:

```php
./target/release/calculator <first-argument> <operator> <second-argument>
```
For example, if you want to add 2 and 3, you would run the following command:

```bash
./target/release/calculator 2 + 3
```
The calculator will output the result of the operation, which in this case would be 5.

Code Explanation
The calculator code is located in the `main.rs` file in the root directory of the repository. Here's a brief explanation of what the code does:

The program takes in three arguments from the command line: the first number, the operator, and the second number.

The operator is parsed from a string to a character using the `.chars().next().unwrap()` method.

The `operate` function is called with the operator and two number arguments, and it returns the result of the operation.

The result is printed to the console.

The `operate` function uses a `match` statement to perform the requested operation based on the operator passed to it. If an invalid operator is passed, the function will panic with an error message.

Conclusion
This is a simple calculator program written in Rust that demonstrates some basic Rust programming concepts, such as pattern matching and error handling. With this program, you can quickly perform calculations from the command line without having to open a separate application.
