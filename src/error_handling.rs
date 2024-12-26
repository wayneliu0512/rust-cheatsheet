use std::fs::File;
use std::io::{self, Read};

pub fn demonstrate() {
    let greeting_file_result = File::open("target/hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("target/hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    dbg!(greeting_file);

    // The above code can be simplified using closures
    let greeting_file = File::open("target/hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("target/hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    dbg!(greeting_file);

    // The above code can be further simplified using unwrap_or_else
    let greeting_file = File::open("target/hello.txt").unwrap();
    dbg!(greeting_file);
    let greeting_file = File::open("target/hello.txt").expect("Failed to open the file");
    dbg!(greeting_file);

    // Propagating errors
    dbg!(propagate_error_result().unwrap());
    dbg!(propagate_error_option("hello").unwrap());
}

fn propagate_error_result() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("target/hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn propagate_error_option(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}