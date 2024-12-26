use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors_logs = extract_errors(text.as_str());
    fs::write("errors.txt", errors_logs.join("\n"))?;
    Ok(())

    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errros.txt")

    // match fs::read_to_string("logs.txt") {
    //     Ok(value) => {
    //         let error_logs = extract_errors(value.as_str());
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(error) => println!("Error when writing errors.txt: {}", error),
    //         }
    //     }
    //     Err(error) => {
    //         println!("Failed to read file: {}", error);
    //     }
    // }
}

fn extract_errors(text: &str) -> Vec<String> {
    let split = text.split("\n");
    let mut results = vec![];
    for line in split {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}
