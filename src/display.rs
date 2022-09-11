use std::fmt::Display;

use colored::Colorize;

use crate::{Tile, MaybeTile, Board, TileKind, Direction};

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for MaybeTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeTile::Undecided(_) => write!(f, "?"),
            MaybeTile::Decided(kind) => write!(f, "{}", kind),
        }
    }
}

impl Display for Board {
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

impl Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            TileKind::Water => "~".blue(),
            TileKind::Ground => "O".bold().on_green(),
            TileKind::Tree => "B".green(),
            TileKind::House(dir) => match dir {
                Direction::North => "#".magenta().on_blue(),
                Direction::East => "#".magenta().on_green(),
                Direction::South => "#".magenta(),
                Direction::West => "#".magenta().on_yellow(),
            },
            TileKind::Road => "-".purple(),
            TileKind::Hut => "v".red(),
            TileKind::Mountain => "X".on_red(),
            TileKind::Sand => "~".yellow(),
        };
        write!(f, "{}", character)
    }
}
