
fn generate_word() -> String {
    let word = String::from("WORLD");
    word
}

fn main() {
    println!("Word game");
    let word = generate_word();

    println!("Word: {}", word);
}
