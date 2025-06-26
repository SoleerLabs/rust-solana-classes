fn main() {
    println!("Solana Bootcamp Exercise");
    println!("========================");
    
    // Assignment Part 1: Display your Solana public key
    // Replace "YOUR_PUBKEY_HERE" with your actual public key from `solana address` command
    let my_pubkey = "DszqTSTQp1AGKMQkiQyHkAgycb9CrWmx4CKBfc26116c";
    println!("My Solana address: {}", my_pubkey);
    
    // Assignment Part 2: Mock balance checker for airdrop eligibility
    let balance = 2.5;
    println!("\nCurrent SOL balance: {}", balance);
    
    // Check airdrop eligibility based on balance
    check_airdrop_eligibility(balance);
}

fn check_airdrop_eligibility(balance: f64) {
    println!("\n--- Airdrop Eligibility Check ---");
    
    if balance < 1.0 {
        println!("Eligible for airdrop! Your balance is low.");
    } else if balance < 5.0 {
        println!("Partially eligible - you might qualify for a smaller airdrop.");
    } else {
        println!("Not eligible for airdrop - your balance is sufficient.");
    }
    
    println!("Eligibility based on balance threshold of 1.0 SOL");
}