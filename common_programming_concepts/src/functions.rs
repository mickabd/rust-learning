fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_param(100);
    print_labeled_measurement(5, 'h');

    let x = get_value();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    // This will cause an error because let y = 6 is a statement and not an expression.
    // let x = (let y = 6);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn get_value() -> i8 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // This is an expression.
    // x + 1; // This is a statement. -> this will cause an error.

}
