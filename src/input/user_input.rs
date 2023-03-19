pub fn user_input() {
    println!("Enter something: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You entered: {}", input);
}