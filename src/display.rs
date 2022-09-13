use std::fmt::Display;

use crate::{MaybeTile, Board, Tile, Direction};

impl<T> Display for MaybeTile<T>
where
    T: Display + Tile
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeTile::Undecided(_) => write!(f, "?"),
            MaybeTile::Decided(kind) => write!(f, "{}", kind),
        }
    }
}

impl<T> Display for Board<T>
where
    T: Display + Tile
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for col in row {
                write!(f, "{}", col)?
            }
            write!(f, "\n")?
        };

        std::fmt::Result::Ok(())
    }
}
