use std::io::stdin;


fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failuuuuure");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    println!("Hello, what's your name ?");

    let your_name = what_is_your_name();

    println!("{:?}", your_name);
}
