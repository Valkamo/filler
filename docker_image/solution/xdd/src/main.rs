use std::io;

fn main() {
    // Read player information
    let mut player_info = String::new();
    io::stdin().read_line(&mut player_info).expect("Failed to read player info");

    // Check if player_info contains "Answer" to skip printing it
    if !player_info.contains("Answer") {
        // println!("{}", player_info.trim()); // Printing player info (optional)
    }

    // Skip the first row with x coordinates
    let mut x_coordinates = String::new();
    io::stdin().read_line(&mut x_coordinates).expect("Failed to read x coordinates");

    // Read Anfield into a 2D vector, skipping first row and column
    let mut anfield = vec![];
    for i in 0..15 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read Anfield line");
        // Skip the three characters in each line, which contain the numbers and spaces
        let row: Vec<String> = line
            .trim()
            .chars()
            .skip(3)
            .filter(|&c| c != ' ')
            .map(|c| c.to_string())
            .collect();
        anfield.push(row);
    }

    // Find the position of '@' on the Anfield
    if
        let Some(row) = anfield
            .iter()
            .position(|line| !line.is_empty() && line.join("").contains("@"))
    {
        if let Some(column) = anfield[row].iter().position(|c| c == "@") {
            println!("Found '@' at position: ({}, {})", column, row - 1);
        }
    }
}
