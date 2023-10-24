use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("xd.log").unwrap();
    let mut anfield: Vec<Vec<char>> = Vec::new();
    let mut piece: Vec<Vec<char>> = Vec::new();

    // writeln!(file, "xddd").expect("Failed to write to file");

    let mut lines = stdin.lock().lines();
    loop {
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
                        let chars: Vec<char> = board_line[4..].chars().collect();
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

                let position = place_piece_on_board(&anfield, &piece);
                if let Some((y, x)) = position {
                    writeln!(file, "Printing!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {} {}", x, y).expect(
                        "Failed to write to file"
                    );
                    println!("{} {}", x, y);
                    io::stdout().flush().unwrap();
                    // clear the piece and anfield
                    piece.clear();
                    anfield.clear();
                } else {
                    writeln!(file, "Printing 0 0").expect("Failed to write to file");
                    println!("0 0");
                    io::stdout().flush().unwrap();
                    // clear the piece and anfield
                    piece.clear();
                    anfield.clear();
                }
            }
        }
    }
}

fn can_place_piece(board: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("xddd.log")
        .unwrap();
    let mut overlap_count = 0;

    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            let board_x = x + i;
            let board_y = y + j;

            // Boundary check
            if board_x >= board.len() || board_y >= board[0].len() {
                if piece[i][j] != '.' {
                    // Only consider non-dot parts of the piece for boundary checks
                    writeln!(file, "boundary check failed x: {}, y:{}", x, y).expect(
                        "Failed to write to file"
                    );
                    return false;
                }
                continue;
            }

            // overlap with enemy check
            if
                piece[i][j] != '.' &&
                (board[board_x][board_y] == '$' || board[board_x][board_y] == 's')
            {
                writeln!(file, "overlap with enemy x: {}, y:{}", x, y).expect(
                    "Failed to write to file"
                );
                return false;
            }

            // Connection check
            if
                piece[i][j] != '.' &&
                (board[board_x][board_y] == '@' || board[board_x][board_y] == 'a')
            {
                writeln!(file, "overlap x: {}, y:{}", x, y).expect("Failed to write to file");
                overlap_count += 1;
            }
        }
    }

    writeln!(file, "overlap_count: {}, x: {}, y: {}", overlap_count, x, y).expect(
        "Failed to write to file"
    );

    overlap_count == 1
}

fn place_piece_on_board(board: &Vec<Vec<char>>, piece: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("xdd.log").unwrap();
    writeln!(file, "place_piece_on_board").expect("Failed to write to file");
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] == '@' {
                writeln!(file, "board x: {}, y: {} on @", x, y).expect("Failed to write to file");
            }
            // write the coordinates to the log file
            writeln!(file, "testing x: {}, y: {}", x, y).expect("Failed to write to file");
            if can_place_piece(&board, &piece, x, y) {
                writeln!(file, "can_place_piece").expect("Failed to write to file");
                // Place the piece on the board at (x, y)
                return Some((x, y));
            }
        }
    }
    writeln!(file, "cannot_place_piece").expect("Failed to write to file");
    None
}
