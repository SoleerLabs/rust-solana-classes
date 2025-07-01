use std::collections::HashMap;
use std::io;

fn main() {
    let mut sentence = String::new();
    println!("Enter a new sentence:");
    io::stdin().read_line(&mut sentence).expect("Failed to read input");
    //split thye sentence into seperate words and store them in a vector aka array of strink aka aka arays
    let words: Vec<String> = sentence.to_lowercase().split_whitespace().map(|w| w.to_string()).collect();
    // Count the frequency of each word using a hash map aka map aka aka set aka aka aka dictionary
    let mut word_count: HashMap<String, i32> = HashMap::new();
    for word in &words {
        *word_count.entry(word.clone()).or_insert(0) += 1;
    }
    //lets do some searching 
    let mut search_word = String::new();
    println!("\nEnter a word to check its frequency:");
    io::stdin() .read_line(&mut search_word).expect("Failed to read input");
    let search_word = search_word.trim().to_lowercase();
    match word_count.get(&search_word) {
        Some(count) => println!("'{}' appears {} times.", search_word, count),
        None => println!("'{}' does not appear in the sentence.", search_word),
    }
}


// - Takes a sentence from the user.

// - Splits it into words.

// - Stores the words in a Vec<String>.

// - Counts how many times a specific word appears.