use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let file = File::open("test.txt");
    let file = match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("test.txt") {
            Ok(file) => file,
            Err(_) => panic!("Fuck!"),
        },
        Err(error) => panic!("{:?}", error),
    };
}
