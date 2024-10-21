// This will fail to compile because x is immutable by default.
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// Solution:
fn main() {
    let mut x = 5; // let x = 5 will cause an error if x is not mutable.;
    println!("The value of x is: {x}");
    x = 6; // This line will cause an error if x is not mutable.
    println!("The value of x is: {x}");

    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let r2 = &mut s; // This will cause an error because s is already borrowed as mutable.
}
