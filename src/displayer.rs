use std::io;

pub fn write(message : &str){
    println!("{}", message);
}

pub fn read_string(message : &str) -> String {
    let mut input = String::new();

    println!("{}", message);

    match io::stdin().read_line(&mut input) {
        Ok(_) => print!(""),
        Err(_e) => println!("{}", _e),
    };

    return input;
}