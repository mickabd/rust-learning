fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Pass a reference by using &.

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello"); // Mutable string.

    change(&mut s); // Pass a mutable reference by using &mut.

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// This function will not compile because it tries to modify a borrowed value.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// This function will compile because it takes a mutable reference.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
