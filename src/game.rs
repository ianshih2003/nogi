use crate::error::ErrorWrapper;

pub type MailBoxBoard = [[Piece; 8]; 8];

#[derive(PartialEq, Debug)]
pub struct ChessGame {
    board: MailBoxBoard,
    turn: Color,
    white_castling: Castling,
    black_castling: Castling,
    en_passant: Option<(usize, usize)>,
    halfmoves: usize,
    fullmoves: usize,
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

#[derive(Clone, Copy, PartialEq, Debug)]

pub enum Castling {
    Both,
    QueenSide,
    KingSide,
    None,
}

pub fn convert_chess_coordinates(coordinates: &str) -> Result<(usize, usize), ErrorWrapper> {
    let [rank, file] = coordinates.chars().collect::<Vec<char>>()[..2] else {
        return Err(ErrorWrapper::InvalidCoordinates);
    };

    let rank = match rank {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(ErrorWrapper::InvalidCoordinates),
    };

    let file = 8
        - (file
            .to_digit(10)
            .ok_or_else(|| ErrorWrapper::InvalidCoordinates))? as usize;

    if file >= 8 {
        return Err(ErrorWrapper::InvalidCoordinates);
    }
    Ok((rank, file))
}

impl ChessGame {
    pub fn new(
        board: MailBoxBoard,
        turn: Color,
        white_castling: Castling,
        black_castling: Castling,
        en_passant: Option<(usize, usize)>,
        halfmoves: usize,
        fullmoves: usize,
    ) -> ChessGame {
        ChessGame {
            board,
            turn,
            white_castling,
            black_castling,
            en_passant,
            halfmoves,
            fullmoves,
        }
    }
}
