fn main() {
    let board = [
        ['r','n','b','q','k','b','n','r'],
        ['p','p','p','p','p','p','p','p'],
        ['.','P','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['P','P','P','P','P','P','P','P'],
        ['R','N','B','Q','K','B','N','R']
    ];

    let files = ['a','b','c','d','e','f','g','h'];

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{} ", board[i][j]);
        }
        print!("\n")
    }

    fn generate_move_pawn_forward(board:[[char; 8]; 8], files:[char; 8]) {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'p' {
                    if i < 7 && board[i+1][j] == '.' {
                        let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                        let new_coordinate = files[j].to_string() + &(board[i].len() - i - 1).to_string();
                        println!("{} -> {}", coordinate, new_coordinate);
                    }
                }

                if board[i][j] == 'P' {
                    if i > 0 && board[i-1][j] == '.' {
                        let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                        let new_coordinate = files[j].to_string() + &(board[i].len() - i + 1).to_string();
                        println!("{} -> {}", coordinate, new_coordinate);
                    }
                }

                if board[i][j] == 'p' && i == 1 {
                    if i < 7 && board[i+1][j] == '.' && board[i+2][j] == '.' {
                        let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                        let new_coordinate = files[j].to_string() + &(board[i].len() - i - 2).to_string();
                        println!("{} -> {}", coordinate, new_coordinate);
                    }
                }

                if board[i][j] == 'P' && i == 6 {
                    if i > 0 && board[i-1][j] == '.' && board[i-2][j] == '.' {
                        let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                        let new_coordinate = files[j].to_string() + &(board[i].len() - i + 2).to_string();
                        println!("{} -> {}", coordinate, new_coordinate);
                    }
                }
            }
        }
    }

    fn generate_pawn_capture_diagonal(board: [[char; 8]; 8], files: [char; 8]) {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'P' && i > 0 && j > 0 && board[i-1][j-1].is_lowercase() {
                    let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                    let new_coordinate = files[j-1].to_string() + &(board[i].len() - i + 1).to_string();
                    println!("{} -> {}", coordinate, new_coordinate);
                }

                if board[i][j] == 'P' && i > 0 && j < 7 && board[i-1][j+1].is_lowercase() {
                    let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                    let new_coordinate = files[j+1].to_string() + &(board[i].len() - i + 1).to_string();
                    println!("{} -> {}", coordinate, new_coordinate);
                }

                if board[i][j] == 'p' && i < 7 && j > 0 && board[i+1][j-1].is_uppercase() {
                    let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                    let new_coordinate = files[j-1].to_string() + &(board[i].len() - i - 1).to_string();
                    println!("{} -> {}", coordinate, new_coordinate);
                }

                if board[i][j] == 'p' && i < 7 && j < 7 && board[i+1][j+1].is_uppercase() {
                    let coordinate = files[j].to_string() + &(board[i].len() - i).to_string();
                    let new_coordinate = files[j+1].to_string() + &(board[i].len() - i - 1).to_string();
                    println!("{} -> {}", coordinate, new_coordinate);
                }
            }
        }
    }

    //generate_move_pawn_forward(board, files);
    generate_pawn_capture_diagonal(board, files);
}
