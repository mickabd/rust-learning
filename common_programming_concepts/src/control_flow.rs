fn main() {
    condition(5);
    condition(4);
    compare_to_zero(8);
    multiple_condition();
    if_in_let_statement();
    loop_();
    returning_values_from_loop();
    multiple_loop_with_break();
    while_();
    loop_with_index_in_list();
    loop_in_list();
    countdown();
}

fn condition(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// This will fail to compile because number is not a boolean.
// fn error() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

fn compare_to_zero(number: i32) {
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn multiple_condition() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// This will fail to compile because the types of the values in the if and else blocks are different.
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// This will loop forever.
fn loop_() {
    loop {
        println!("again!");
        break; // This will break the loop.
    }
}

fn returning_values_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn multiple_loop_with_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_index_in_list() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_in_list() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}