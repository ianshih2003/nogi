use crate::error::ErrorWrapper;

pub struct ChessGame {
    board: [[Piece; 8]; 8],
    turn: Color,
    // white_castling: Castling,
    // black_castling: Castling,
    // en_passant: Option<(usize, usize)>,
    // halfmove: u8,
    // fullmove: u16,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    Empty,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

pub enum Castling {
    Both,
    QueenSide,
    KingSide,
}

fn fen_piece_to_piece(fen_piece: char) -> Piece {
    let color = if fen_piece.is_uppercase() {
        Color::White
    } else {
        Color::Black
    };

    match fen_piece.to_ascii_lowercase() {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => Piece::Empty,
        'p' => Piece::Pawn(color),
        'n' => Piece::Knight(color),
        'b' => Piece::Bishop(color),
        'r' => Piece::Rook(color),
        'q' => Piece::Queen(color),
        'k' => Piece::King(color),
        _ => Piece::Empty,
    }
}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame {
            board: [[Piece::Empty; 8]; 8],
            turn: Color::White,
            // white_castling: Castling::Both,
            // black_castling: Castling::Both,
            // en_passant: None,
            // halfmove: 1,
            // fullmove: 0,
        }
    }

    pub fn set_turn(&mut self, turn: Color) {
        self.turn = turn;
    }

    pub fn set_piece(&mut self, piece: Piece, file: usize, rank: usize) {
        self.board[file][rank] = piece;
    }

    pub fn create_from_fen(fen: &str) -> Result<ChessGame, ErrorWrapper> {
        let mut new_game = ChessGame::new();
        let [piece_placement, active_color, _castling] =
            fen.split(" ").collect::<Vec<&str>>()[0..3]
        else {
            return Err(ErrorWrapper::InvalidFen);
        };

        let color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ErrorWrapper::InvalidFen),
        };
        new_game.set_turn(color);

        let mut file = 0;
        let mut rank = 0;
        for piece in piece_placement.chars() {
            if piece == '/' {
                file += 1;
                rank = 0;
                continue;
            }

            if let Some(empty_squares) = piece.to_digit(10) {
                let empty_squares = empty_squares as usize;

                for i in 0..empty_squares {
                    new_game.set_piece(Piece::Empty, file, rank + i)
                }

                rank += empty_squares;
                continue;
            }

            let piece = fen_piece_to_piece(piece);

            new_game.set_piece(piece, file, rank);

            rank += 1
        }

        Ok(new_game)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::game::{Color, Piece};

    use super::ChessGame;

    const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    pub fn new_game_from_starting_position_fen() {
        let game = ChessGame::create_from_fen(STARTING_POSITION).unwrap();

        let black = Color::Black;
        let white = Color::White;
        assert_eq!(
            game.board,
            [
                [
                    Piece::Rook(black),
                    Piece::Knight(black),
                    Piece::Bishop(black),
                    Piece::Queen(black),
                    Piece::King(black),
                    Piece::Bishop(black),
                    Piece::Knight(black),
                    Piece::Rook(black),
                ],
                [
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                    Piece::Pawn(black),
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                    Piece::Pawn(white),
                ],
                [
                    Piece::Rook(white),
                    Piece::Knight(white),
                    Piece::Bishop(white),
                    Piece::Queen(white),
                    Piece::King(white),
                    Piece::Bishop(white),
                    Piece::Knight(white),
                    Piece::Rook(white),
                ],
            ]
        )
    }
}
