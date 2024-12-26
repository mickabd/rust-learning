fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&colors);

    let found_color = find_color_or(&colors, "re", "orange");
    println!("{}", found_color);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);

    let uppercase = &to_uppercase(&colors);
    print_elements(&uppercase);

    shorten_strings(&mut colors);
    print_elements(&colors);

    let vec_a = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut vec_b = vec![String::from("orange"), String::from("yellow")];
    move_elements(vec_a, &mut vec_b);
    print_elements(&vec_b);
}

fn shorten_strings(elements: &mut [String]) {
    // for el in elements {
    //     el.truncate(1);
    // }
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn print_elements(elements: &[String]) {
    // Option 1: use a for loop.
    // for element in elements {
    //     println!("{}", element);
    // }

    // Option 2: use adapter and consumer.
    elements.iter().for_each(|elmt| println!("{}", elmt));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|f| f.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|x| x.contains(search))
        .map_or(fallback.to_string(), |x| x.to_string())
}
