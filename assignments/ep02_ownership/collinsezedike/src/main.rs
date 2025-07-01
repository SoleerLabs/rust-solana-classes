use std::io;

fn main() {
    // For ownership and borrowing exercise we do this
    // Build a Word Counter
    // You will write a program that:

    // - Takes a sentence from the user.
    println!("\nWelcome to the Word Counter!");
    println!("Please enter a sentence, and I will count the words for you.\n");
    let mut user_sentence = String::new();
    io::stdin().read_line(&mut user_sentence).expect("Failed to get user input");

    // - Splits it into words.
    // - Stores the words in a Vec<String>.
    let words: Vec<String> = user_sentence
        .trim()
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    // - Counts how many times a specific word appears.
    println!("\nNow, please enter a word to count its occurrences:\n");
    let mut user_word = String::new();
    io::stdin().read_line(&mut user_word).expect("Failed to get user input");

    let word_to_count = user_word.trim();
    let mut count = 0;
    for word in &words {
        if word == word_to_count {
            count += 1;
        }
    }

    println!(
        "\n'{}' appears {} times in the sentence '{}'",
        word_to_count,
        count,
        user_sentence.trim()
    );
}
