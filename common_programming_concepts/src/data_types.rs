use std::io;

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // boolean
    let _t = true;
    let _f: bool = false;

    // character
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // Array
    let _a = [1, 2, 3, 4, 5];
    // This will cause an error because the array is of type i32.
    // let _error_list = [1, 2, 3, 4, 5.0];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // This will create an array of 5 elements with the value 3.

    // Accessing Array Elements
    let _first = a[0];
    let _second = a[1];

    _invalid_array_element_access();
}

fn _invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
