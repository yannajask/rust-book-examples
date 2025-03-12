fn main() {
    println!("Hello world!");

    // Calling a function with a parameter
    another_function(5);

    // Calling a function with multiple parameters
    print_labeled_measurement(5, 'h');

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Calling a function with a return value
    let x = five();

    println!("The value of x is: {x}");

    // Calling a function with a parameter and return value
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}