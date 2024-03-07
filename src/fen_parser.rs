use crate::{
    error::ErrorWrapper,
    game::{convert_chess_coordinates, Castling, ChessGame, Color, MailBoxBoard, Piece},
};

pub const BLACK: Color = Color::Black;
pub const WHITE: Color = Color::White;

fn fen_piece_to_piece(fen_piece: char) -> Result<Piece, ErrorWrapper> {
    let color = if fen_piece.is_uppercase() {
        Color::White
    } else {
        Color::Black
    };

    Ok(match fen_piece.to_ascii_lowercase() {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => Piece::Empty,
        'p' => Piece::Pawn(color),
        'n' => Piece::Knight(color),
        'b' => Piece::Bishop(color),
        'r' => Piece::Rook(color),
        'q' => Piece::Queen(color),
        'k' => Piece::King(color),
        _ => return Err(ErrorWrapper::InvalidFen),
    })
}

pub fn parse_fen(fen: &str) -> Result<(&str, &str, &str, &str, &str, &str), ErrorWrapper> {
    let [piece_placement, active_color, castling, en_passant, halfmoves, fullmoves] =
        fen.split(" ").collect::<Vec<&str>>()[0..6]
    else {
        return Err(ErrorWrapper::InvalidFen);
    };

    Ok((
        piece_placement,
        active_color,
        castling,
        en_passant,
        halfmoves,
        fullmoves,
    ))
}

pub fn create_from_fen(fen: &str) -> Result<ChessGame, ErrorWrapper> {
    let (piece_placement, active_color, castling, en_passant, halfmoves, fullmoves) =
        parse_fen(fen)?;

    let color = parse_active_color(active_color)?;

    let board = parse_piece_placement(piece_placement)?;

    let (white_castling, black_castling) = parse_castling_rights(castling)?;

    let en_passant = parse_en_passant_square(en_passant)?;

    let halfmoves = parse_moves(halfmoves)?;
    let fullmoves = parse_moves(fullmoves)?;

    Ok(ChessGame::new(
        board,
        color,
        white_castling,
        black_castling,
        en_passant,
        halfmoves,
        fullmoves,
    ))
}

fn parse_piece_placement(piece_placement: &str) -> Result<MailBoxBoard, ErrorWrapper> {
    let mut file: usize = 0;
    let mut rank: usize = 0;
    let mut board: MailBoxBoard = [[Piece::Empty; 8]; 8];
    for piece in piece_placement.chars() {
        if piece == '/' {
            file += 1;
            rank = 0;
            continue;
        }

        if let Some(empty_squares) = piece.to_digit(10) {
            let empty_squares = empty_squares as usize;

            rank += empty_squares;
            continue;
        }

        let piece = fen_piece_to_piece(piece)?;

        board[file][rank] = piece;

        rank += 1
    }
    Ok(board)
}

pub fn parse_active_color(active_color: &str) -> Result<Color, ErrorWrapper> {
    Ok(match active_color {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(ErrorWrapper::InvalidFen),
    })
}

pub fn parse_castling_rights(castling: &str) -> Result<(Castling, Castling), ErrorWrapper> {
    let mut white_castling = Castling::None;
    let mut black_castling = Castling::None;
    for char in castling.chars() {
        if char == '-' {
            return Ok((white_castling, black_castling));
        }

        let castling = match char.to_ascii_lowercase() {
            'k' => Castling::KingSide,
            'q' => Castling::QueenSide,
            _ => return Err(ErrorWrapper::InvalidFen),
        };

        let target_castling = if char.is_lowercase() {
            &mut black_castling
        } else {
            &mut white_castling
        };

        *target_castling = match (*target_castling, castling) {
            (Castling::KingSide, Castling::QueenSide) => Castling::Both,
            (_, new_castling) => new_castling,
        };
    }

    Ok((white_castling, black_castling))
}

fn parse_en_passant_square(en_passant: &str) -> Result<Option<(usize, usize)>, ErrorWrapper> {
    match en_passant {
        "-" => Ok(None),
        en_passant_square => {
            convert_chess_coordinates(en_passant_square).map(|square| Some(square))
        }
    }
}

fn parse_moves(moves: &str) -> Result<usize, ErrorWrapper> {
    moves
        .parse::<usize>()
        .map_err(|_e| ErrorWrapper::InvalidNumber)
}

#[cfg(test)]
pub mod tests {
    use crate::{
        error::ErrorWrapper,
        fen_parser::{parse_en_passant_square, parse_moves, parse_piece_placement, BLACK, WHITE},
        game::{Castling, ChessGame},
        test_boards::{BOARD2, BOARD3, STARTING_BOARD},
    };

    use super::{create_from_fen, parse_active_color, parse_castling_rights};

    const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    pub fn new_game_from_starting_position_fen() {
        let starting_position = STARTING_POSITION.split(" ").collect::<Vec<&str>>()[0];
        let board = parse_piece_placement(starting_position).unwrap();

        assert_eq!(board, STARTING_BOARD)
    }

    #[test]
    fn board_2_piece_placement_is_correct() {
        let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8";
        let result = parse_piece_placement(fen).unwrap();

        assert_eq!(result, BOARD2)
    }

    #[test]
    fn board_3_piece_placement_is_correct() {
        let fen = "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1";
        let result = parse_piece_placement(fen).unwrap();

        assert_eq!(result, BOARD3)
    }

    #[test]
    fn black_active_color_is_correct() {
        let active_color = "b";

        let result = parse_active_color(active_color).unwrap();

        assert_eq!(result, BLACK);
    }

    #[test]
    fn white_active_color_is_correct() {
        let active_color = "w";

        let result = parse_active_color(active_color).unwrap();

        assert_eq!(result, WHITE);
    }

    #[test]
    fn invalid_active_color_throws_error() {
        let active_color = "a";

        let result = parse_active_color(active_color);

        assert_eq!(result, Err(ErrorWrapper::InvalidFen));
    }

    #[test]
    fn complete_castling_rights() {
        let castling = "KQkq";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::Both, Castling::Both));
    }

    #[test]
    fn white_kingside_castling_rights() {
        let castling = "Kkq";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::KingSide, Castling::Both));
    }

    #[test]
    fn white_queenside_castling_rights() {
        let castling = "Qkq";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::QueenSide, Castling::Both));
    }

    #[test]
    fn black_kingside_castling_rights() {
        let castling = "k";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::None, Castling::KingSide));
    }

    #[test]
    fn black_queenside_castling_rights() {
        let castling = "KQq";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::Both, Castling::QueenSide));
    }

    #[test]
    fn no_castling_rights() {
        let castling = "-";

        let result = parse_castling_rights(castling).unwrap();

        assert_eq!(result, (Castling::None, Castling::None));
    }

    #[test]
    fn en_passant_square() {
        let square = "e3";

        let result = parse_en_passant_square(square).unwrap().unwrap();

        assert_eq!(result, (4, 5));
    }

    #[test]
    fn invalid_en_passant_square() {
        let square = "o1";

        let result = parse_en_passant_square(square);

        assert_eq!(result, Err(ErrorWrapper::InvalidCoordinates));
    }

    #[test]
    fn no_en_passant() {
        let square = "-";

        let result = parse_en_passant_square(square).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn moves() {
        let moves = "50";

        let result = parse_moves(moves).unwrap();

        assert_eq!(result, 50);
    }

    #[test]
    fn invalid_moves() {
        let moves = "asdas";

        let result = parse_moves(moves);

        assert_eq!(result, Err(ErrorWrapper::InvalidNumber));
    }

    #[test]
    fn full_game_generation() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let result = create_from_fen(fen).unwrap();

        assert_eq!(
            result,
            ChessGame::new(
                STARTING_BOARD,
                WHITE,
                Castling::Both,
                Castling::Both,
                None,
                0,
                1
            )
        )
    }

    #[test]
    fn end_game_generation() {
        let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";

        let result = create_from_fen(fen).unwrap();

        assert_eq!(
            result,
            ChessGame::new(BOARD2, BLACK, Castling::None, Castling::None, None, 99, 50)
        )
    }
}
