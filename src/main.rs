use std::io::stdin;

fn main() {
   println!("Welcome to word spammer!");
   println!("Please input the string you would like to spam:");

let mut input = String::new();

   std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");

loop {
    println!("{}", &input);
}
}
