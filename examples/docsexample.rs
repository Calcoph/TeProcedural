use std::{collections::HashSet, fmt::Display};

fn main() {
    // First, we choose the size of the board
    let width = 10;
    let length = 10;
    let height = 1;
    // What kind of board will be generated is dictated by <ChessTile> We will define ChessTile later
    let mut board = procedural::Board::<ChessTile>::new(width, length, height);
    //board.set_tile(procedural::MaybeTile::Decided(ChessTile::Black), 0, 0, 0).unwrap(); // Uncomment this line so top-left corner is black
    //board.set_tile(procedural::MaybeTile::Decided(ChessTile::White), 0, 0, 0).unwrap(); // Uncomment this line so top-left corner is white
    // After setting (or not) the initial state of the board, we can generate the rest of it. With Board::generate(&mut self).
    board.generate().unwrap();
    // Because ChessTile implements Display, we can print it.
    println!("{board}")
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)] // Derives required to implement procedural::Tile
enum ChessTile {
    // We are going to generate a simple chess board, which only has 2 kind of tiles. Black and white.
    Black,
    White
}

impl procedural::Tile for ChessTile {
    type Direction = ChessDirection; // ChessDirection defined later.

    fn all() -> std::collections::HashSet<Self> {
        // A set containing all the possible tiles
        let mut h = HashSet::new();
        h.insert(ChessTile::Black);
        h.insert(ChessTile::White);
        h
    }

    #[allow(unused_variables)]
    fn possibles(layer: usize) -> std::collections::HashSet<Self> {
        // Same as Tile::all(). But we can choose to have different possibilities depending on the layer.
        // Since a chess board is 2D, all() and possibles(layer) are exactly the same.
        let mut h = HashSet::new();
        h.insert(ChessTile::Black);
        h.insert(ChessTile::White);
        h
    }

    fn get_rules(&self) -> Box<dyn Fn(&Self,Self::Direction) -> bool+ '_> {
        // The rules are what will make the chess pattern appear.
        match self {
            ChessTile::Black => Box::new(|tile: &ChessTile, _direction: ChessDirection| match tile {
                ChessTile::Black => false,
                ChessTile::White => true, // Black tiles can only be next to white tiles.
            }),
            ChessTile::White => Box::new(|tile: &ChessTile, _direction: ChessDirection| match tile {
                ChessTile::Black => true, // White tiles can only be next to black tiles.
                ChessTile::White => false,
            }),
        }
    }

    #[allow(unused_variables)]
    fn get_distribution(&self, layer: usize) -> u32 {
        // Since white tiles are as common as black tiles, they both return the same number.
        // Same results would be achieved by returning any other number,
        // but for performance reasons, we will return 1.
        1
    }
}

impl Display for ChessTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessTile::Black => write!(f, " "),
            ChessTile::White => write!(f, "#"),
        }
    }
}

#[derive(Clone, Copy)]
enum ChessDirection {
    // Since chess is a 2D board and we don't need to look diagonally,
    // 4 directions are enough. 2 for horizontal, 2 for vertical.
    North,
    East,
    South,
    West
}

impl procedural::Direction for ChessDirection {
    fn all() -> Vec<Self> {
        // Same as Tile::all()
        vec![ChessDirection::North, ChessDirection::East, ChessDirection::South, ChessDirection::West]
    }

    #[allow(unused_variables)]
    fn neighbour(
        &self,
        row: usize,
        col: usize,
        layer: usize,
        width: u32,
        length: u32,
        height: u32
    ) -> Result<(usize, usize, usize), procedural::CoordError> {
        // Converts the input coordinates according to the direction.
        // If the output would be out of the board, we return procedural::CoordError instead
        match self {
            ChessDirection::North => match row {
                0 => Err(procedural::CoordError),
                _ => Ok((row-1, col, layer))
            },
            ChessDirection::East => match col {
                x if x+1 >= width as usize => Err(procedural::CoordError),
                _ => Ok((row, col+1, layer))
            },
            ChessDirection::South => match row {
                y if y+1 >= length as usize => Err(procedural::CoordError),
                _ => Ok((row+1, col, layer))
            },
            ChessDirection::West => match col {
                0 => Err(procedural::CoordError),
                _ => Ok((row, col-1, layer))
            },
        }
    }

    fn opposite(&self) -> Self {
        // All directions must have an opposite so the algorithm works correctly.
        match self {
            ChessDirection::North => ChessDirection::South,
            ChessDirection::East => ChessDirection::West,
            ChessDirection::South => ChessDirection::North,
            ChessDirection::West => ChessDirection::East,
        }
    }
}