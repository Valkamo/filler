use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("xd.log").unwrap();
    let mut anfield: Vec<Vec<char>> = Vec::new();

    // writeln!(file, "xddd").expect("Failed to write to file");

    let mut lines = stdin.lock().lines();

    while let Some(result) = lines.next() {
        let line = result.expect("Failed to read line");
        // writeln!(file, "{}", line).expect("Failed to write to file");

        if line.starts_with("$$$") {
            let p_parts: Vec<&str> = line.split_whitespace().collect();
            let player: String = p_parts[2].parse().expect("Failed to parse player");
            writeln!(file, "player: {}", player).expect("Failed to write to file");
        }

        if line.starts_with("Anfield") {
            let an_parts: Vec<&str> = line.split_whitespace().collect();
            let an_width: usize = an_parts[1].parse().expect("Failed to parse width");
            let an_height: usize = an_parts[2]
                .trim_end_matches(':')
                .parse()
                .expect("Failed to parse height");

            writeln!(file, "anfield width: {}, height: {}", an_width, an_height).expect(
                "Failed to write to file"
            );

            // Skip the next line which only has the numbers
            let _ = lines.next().expect("Failed to skip numbers line");

            for _ in 0..an_height {
                if let Some(board_line_result) = lines.next() {
                    let board_line = board_line_result.expect("Failed to read board line");
                    let chars: Vec<char> = board_line[5..].chars().collect();
                    anfield.push(chars);
                }
            }

            for row in &anfield {
                let row_str: String = row.iter().collect();
                writeln!(file, "{}", row_str).expect("Failed to write to file");
            }
        }

        if line.starts_with("Piece") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let width: usize = parts[1].parse().expect("Failed to parse width");
            let height: usize = parts[2]
                .trim_end_matches(':')
                .parse()
                .expect("Failed to parse height");

            writeln!(file, "Piece width: {}, height: {}", width, height).expect(
                "Failed to write to file"
            );

            let mut piece: Vec<Vec<char>> = Vec::new();

            for _ in 0..height {
                if let Some(piece_line_result) = lines.next() {
                    let piece_line = piece_line_result.expect("Failed to read piece line");
                    let chars: Vec<char> = piece_line.chars().collect();
                    piece.push(chars);
                }
            }

            // Optionally, print the piece to the log file to verify
            for row in &piece {
                let row_str: String = row.iter().collect();
                writeln!(file, "{}", row_str).expect("Failed to write to file");
            }
        }
    }
}
