fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    area_tuple(rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect1)
    );

    println!("rect1 is {rect1:?}",); // This will cause an error because Rectangle does not implement the Display trait.

    dbg!(&rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
