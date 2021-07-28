use std::io ;

fn main() {
    let arr: [&str; 3] = ["Daksh", "parth", "Anushka"];
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let index: usize = guess
        .trim()
        .parse()
        .expect("Index entered was not a number");
    println!("Friend you got is: {}", arr[index]);
}
