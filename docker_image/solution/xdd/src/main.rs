use std::io;

fn main() {
    // Read player information
    let mut player_info = String::new();
    io::stdin().read_line(&mut player_info).expect("Failed to read player info");

    // Check if player_info contains "Answer" to skip printing it
    if !player_info.contains("Answer") {
        println!("{}", player_info.trim()); // Printing player info (optional)
    }

    // Read piece information
    let mut piece_info = String::new();

    // Read lines until an empty line is encountered
    while io::stdin().read_line(&mut piece_info).expect("Failed to read piece info") > 0 {
        if piece_info.trim().is_empty() {
            println!("Empty line encountered, breaking out of the loop");
            // Break out of the loop when an empty line is encountered
            break;
        }

        // Print the current line of the piece information
        println!("{}", piece_info.trim());

        piece_info.clear(); // Clear the string for the next iteration
    }
}
