fn main() {
    let mut board = [
        ['r','n','b','q','k','b','n','r'],
        ['p','p','p','p','p','p','p','p'],
        ['.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.'],
        ['P','P','P','P','P','P','P','P'],
        ['R','N','B','Q','K','B','N','R']
    ];

    let files = ['a','b','c','d','e','f','g','h'];

    #[derive(Debug, Clone, Copy)]
    struct Move {
        from: (usize, usize),
        to: (usize, usize)
    }

    #[derive(Debug, Clone, Copy)]
    struct Undo {
        captured: char
    }

    fn print_board(board: [[char; 8]; 8]) {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                print!("{} ", board[i][j]);
            }
            print!("\n")
        }
        println!();
    }

    print_board(board);

    fn generate_move_pawn_forward(board: [[char; 8]; 8]) -> Vec<Move> {

        let mut moves = vec![];

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'p' {
                    if i < 7 && board[i+1][j] == '.' {
                        let mv = Move {from: (i, j), to: (i+1, j)};
                        moves.push(mv);
                    }
                }

                if board[i][j] == 'P' {
                    if i > 0 && board[i-1][j] == '.' {
                        let mv = Move {from: (i, j), to: (i-1, j)};
                        moves.push(mv);
                    }
                }

                if board[i][j] == 'p' && i == 1 {
                    if i < 7 && board[i+1][j] == '.' && board[i+2][j] == '.' {
                        let mv = Move {from: (i, j), to: (i+2, j)};
                        moves.push(mv);
                    }
                }

                if board[i][j] == 'P' && i == 6 {
                    if i > 0 && board[i-1][j] == '.' && board[i-2][j] == '.' {
                        let mv = Move {from: (i, j), to: (i-2, j)};
                        moves.push(mv);
                    }
                }
            }
        }

        moves
    }

    fn generate_pawn_capture_diagonal(board: [[char; 8]; 8]) -> Vec<Move> {

        let mut moves = vec![];

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'P' && i > 0 && j > 0 && board[i-1][j-1].is_lowercase() {
                    let mv = Move {from: (i, j), to: (i-1, j-1)};
                    moves.push(mv);
                }

                if board[i][j] == 'P' && i > 0 && j < 7 && board[i-1][j+1].is_lowercase() {
                    let mv = Move {from: (i, j), to: (i-1, j+1)};
                    moves.push(mv);
                }

                if board[i][j] == 'p' && i < 7 && j > 0 && board[i+1][j-1].is_uppercase() {
                    let mv = Move {from: (i, j), to: (i+1, j-1)};
                    moves.push(mv);
                }

                if board[i][j] == 'p' && i < 7 && j < 7 && board[i+1][j+1].is_uppercase() {
                    let mv = Move {from: (i, j), to: (i+1, j+1)};
                    moves.push(mv);
                }
            }
        }

        moves
    }

    fn parse_raw_move(moves: Vec<Move>, board: [[char; 8]; 8], files: [char; 8]) -> Vec<String> {
        
        let mut parsed_moves = vec![];
        
        for i in 0..moves.len() {
            let current_move = &moves[i];
            let file = files[current_move.from.1];
            let rank = board.len() - current_move.from.0;
            let new_file = files[current_move.to.1];
            let new_rank = board.len() - current_move.to.0;

            let coordinate = file.to_string() + &rank.to_string();
            let new_coordinate = new_file.to_string() + &new_rank.to_string();

            let final_move = format!("{} -> {}", coordinate, new_coordinate);
            parsed_moves.push(final_move);
        }

        parsed_moves
    }

    let mut parsed_moves = generate_move_pawn_forward(board);
    parsed_moves.extend(generate_pawn_capture_diagonal(board));

    fn apply_move(mut board: [[char; 8]; 8], mv: Move) -> ([[char; 8]; 8], Undo) {
        let captured = board[mv.to.0][mv.to.1];
        let piece = board[mv.from.0][mv.from.1];
        board[mv.from.0][mv.from.1] = '.';
        board[mv.to.0][mv.to.1] = piece;
        (board, Undo { captured })
    }

    fn undo_move(mut board: [[char; 8]; 8], mv: Move, undo: Undo) -> [[char; 8]; 8] {
        board[mv.from.0][mv.from.1] = board[mv.to.0][mv.to.1];
        board[mv.to.0][mv.to.1] = undo.captured;

        board
    }

    let (new_board, undo) = apply_move(board, parsed_moves[0]);
    print_board(new_board);
    
    let restored_board = undo_move(new_board, parsed_moves[0], undo);
    print_board(restored_board);

}
