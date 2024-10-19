fn main() {
    let mut x = 5; // let x = 5 will cause an error if x is not mutable.;
    println!("The value of x is: {x}");
    x = 6; // This line will cause an error if x is not mutable.
    println!("The value of x is: {x}");
}
