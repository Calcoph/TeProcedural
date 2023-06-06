//! Procedurally generate 2D or 3D maps.
//! 
//! ## Getting Started
//! This example assumes that the "view3d" feature is disabled. For
//! examples using view3d see the examples in the github repo.
//! ```rust
//! use std::{collections::HashSet, fmt::Display};
//! 
//! fn main() {
//!     // First, we choose the size of the board
//!     let width = 10;
//!     let length = 10;
//!     let height = 1;
//!     // What kind of board will be generated is dictated by <ChessTile> We will define ChessTile later
//!     let mut board = procedural::Board::<ChessTile>::new(width, length, height);
//!     //board.set_tile(procedural::MaybeTile::Decided(ChessTile::Black), 0, 0, 0).unwrap(); // Uncomment this line so top-left corner is black
//!     //board.set_tile(procedural::MaybeTile::Decided(ChessTile::White), 0, 0, 0).unwrap(); // Uncomment this line so top-left corner is white
//!     // After setting (or not) the initial state of the board, we can generate the rest of it. With Board::generate(&mut self).
//!     board.generate().unwrap();
//!     // Because ChessTile implements Display, we can print it.
//!     println!("{board}")
//! }
//! 
//! #[derive(Clone, Copy, Hash, PartialEq, Eq)] // Derives required to implement procedural::Tile
//! enum ChessTile {
//!     // We are going to generate a simple chess board, which only has 2 kind of tiles. Black and white.
//!     Black,
//!     White
//! }
//! 
//! impl procedural::Tile for ChessTile {
//!     type Direction = ChessDirection; // ChessDirection defined later.
//! 
//!     fn all() -> std::collections::HashSet<Self> {
//!         // A set containing all the possible tiles
//!         let mut h = HashSet::new();
//!         h.insert(ChessTile::Black);
//!         h.insert(ChessTile::White);
//!         h
//!     }
//! 
//!     #[allow(unused_variables)]
//!     fn possibles(layer: usize) -> std::collections::HashSet<Self> {
//!         // Same as Tile::all(). But we can choose to have different possibilities depending on the layer.
//!         // Since a chess board is 2D, all() and possibles(layer) are exactly the same.
//!         let mut h = HashSet::new();
//!         h.insert(ChessTile::Black);
//!         h.insert(ChessTile::White);
//!         h
//!     }
//! 
//!     fn get_rules(&self) -> Box<dyn Fn(&Self,Self::Direction) -> bool+ '_> {
//!         // The rules are what will make the chess pattern appear.
//!         match self {
//!             ChessTile::Black => Box::new(|tile: &ChessTile, _direction: ChessDirection| match tile {
//!                 ChessTile::Black => false,
//!                 ChessTile::White => true, // Black tiles can only be next to white tiles.
//!             }),
//!             ChessTile::White => Box::new(|tile: &ChessTile, _direction: ChessDirection| match tile {
//!                 ChessTile::Black => true, // White tiles can only be next to black tiles.
//!                 ChessTile::White => false,
//!             }),
//!         }
//!     }
//! 
//!     #[allow(unused_variables)]
//!     fn get_distribution(&self, layer: usize) -> u32 {
//!         // Since white tiles are as common as black tiles, they both return the same number.
//!         // Same results would be achieved by returning any other number,
//!         // but for performance reasons, we will return 1.
//!         1
//!     }
//! }
//! 
//! impl Display for ChessTile {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         match self {
//!             ChessTile::Black => write!(f, " "),
//!             ChessTile::White => write!(f, "#"),
//!         }
//!     }
//! }
//! 
//! #[derive(Clone, Copy)]
//! enum ChessDirection {
//!     // Since chess is a 2D board and we don't need to look diagonally,
//!     // 4 directions are enough. 2 for horizontal, 2 for vertical.
//!     North,
//!     East,
//!     South,
//!     West
//! }
//! 
//! impl procedural::Direction for ChessDirection {
//!     fn all() -> Vec<Self> {
//!         // Same as Tile::all()
//!         vec![ChessDirection::North, ChessDirection::East, ChessDirection::South, ChessDirection::West]
//!     }
//! 
//!     #[allow(unused_variables)]
//!     fn neighbour(
//!         &self,
//!         row: usize,
//!         col: usize,
//!         layer: usize,
//!         width: u32,
//!         length: u32,
//!         height: u32
//!     ) -> Result<(usize, usize, usize), procedural::CoordError> {
//!         // Converts the input coordinates according to the direction.
//!         // If the output would be out of the board, we return procedural::CoordError instead
//!         match self {
//!             ChessDirection::North => match row {
//!                 0 => Err(procedural::CoordError),
//!                 _ => Ok((row-1, col, layer))
//!             },
//!             ChessDirection::East => match col {
//!                 x if x+1 >= width as usize => Err(procedural::CoordError),
//!                 _ => Ok((row, col+1, layer))
//!             },
//!             ChessDirection::South => match row {
//!                 y if y+1 >= length as usize => Err(procedural::CoordError),
//!                 _ => Ok((row+1, col, layer))
//!             },
//!             ChessDirection::West => match col {
//!                 0 => Err(procedural::CoordError),
//!                 _ => Ok((row, col-1, layer))
//!             },
//!         }
//!     }
//! 
//!     fn opposite(&self) -> Self {
//!         // All directions must have an opposite so the algorithm works correctly.
//!         match self {
//!             ChessDirection::North => ChessDirection::South,
//!             ChessDirection::East => ChessDirection::West,
//!             ChessDirection::South => ChessDirection::North,
//!             ChessDirection::West => ChessDirection::East,
//!         }
//!     }
//! }
//! ```

#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashSet};
#[cfg(feature = "view3d")]
use std::path::Path;

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::seq::IteratorRandom;
#[cfg(feature = "view3d")]
use te_renderer::model::ModelVertex;
#[cfg(feature = "view3d")]
use te_renderer::state::GpuState;
#[cfg(feature = "view3d")]
use te_renderer::state::TeState;

mod display;

#[derive(Debug)]
struct DecisionBranch<T>
where
    T: Tile
{
    deciding_coord: (usize, usize, usize),
    tried_tiles: HashSet<T>,
    dead_ends: Vec<(usize, usize, usize)>,
    temp_decided_coords: HashSet<(usize, usize, usize)>
}

impl<T> DecisionBranch<T>
where
    T: Tile
{
    fn new(row: usize, col: usize, layer: usize) -> DecisionBranch<T> {
        DecisionBranch {
            deciding_coord: (row, col, layer),
            tried_tiles: HashSet::new(),
            dead_ends: vec![],
            temp_decided_coords: HashSet::new()
        }
    }
}

/// The status of the current decision branch
pub enum BranchStatus {
    /// The board is complete and no generating has to be done
    Complete,
    /// There is no possible board from this point on, must go back the decision tree and try another branch
    DeadEnd,
    /// This branch leads to a valid board, but not all tiles have been selected yet
    Incomplete,
    /// The current layer is complete, but the board is not, generating must continue.
    CompleteLayer
}

#[derive(Debug)]
/// Due to the rules/directions/tiles/size of this board, there is no combination of tiles that meet all rules
pub struct ImpossibleBoardError;

#[derive(Debug)]
/// Returned instead of a coordinate that would be outside the board
pub struct OutOfBoardError;

#[derive(Debug)]
/// Returned when manually placing tiles that are impossible in the current state of the board.
/// See [Board::set_tile()]
pub enum BadPlacementError{
    /// When trying to place a tile in a position that already has a tile
    TileOccupied,
    /// When trying to place a tile that was already placed
    TileAlreadyPlaced,
    /// When placing a tile is not possible due to the surrounding tiles' rules
    ImpossibleTile,
    /// When not all tiles of a set of possibilities can be placed in a tile, due to surrounding tiles' rules
    NotAllPossible
}

/// Represents the 2D or 3D board that we want to procedurally generate.
pub struct Board<T>
where
    T: Tile
{
    tiles: Vec<Vec<Vec<MaybeTile<T>>>>,
    decision_stack: Vec<DecisionBranch<T>>,
    dead_ends: Vec<(usize, usize, usize)>,
    current_layer: usize,
    width: u32,
    length: u32,
    height: u32
}

impl<T> Board<T>
where
    T: Tile
{
    /// Create an empty board.
    /// The initial state of the board is determined by T::possibles(layer)
    pub fn new(width: u32, length: u32, height: u32) -> Board<T> {
        let mut layers = vec![];
        for cur_layer in 0..height as usize {
            let mut layer = vec![];
            for _ in 0..length {
                let mut row = vec![];
                for _ in 0..width {
                    row.push(MaybeTile::Undecided(T::possibles(cur_layer)))
                }
                layer.push(row)
            }
            layers.push(layer)
        }

        let board = Board {
            tiles: layers,
            decision_stack: vec![],
            dead_ends: vec![],
            width,
            length,
            height,
            current_layer: 0,
        };

        #[cfg(feature = "validate")]
        board.validate();

        board
    }

    #[cfg(feature = "validate")]
    fn validate(&self) {
        let all = T::all();
        let mut names = HashSet::new();
        all.iter().for_each(|tile| {
            if !names.insert(tile.get_name()) {
                println!("WARNING: There is more than one tile with the name `{}`", tile.get_name())
            }
        });
        let directions = T::Direction::all();
        all.iter()
            .flat_map(|tile| all.iter().map(|t| (tile, t)).collect::<Vec<_>>())
            .flat_map(|(tile1, tile2)| directions.iter().map(|direction| (tile1, tile2, direction)).collect::<Vec<_>>())
            .for_each(|(tile1, tile2, direction)| {
                if tile1.get_rules()(tile2, *direction)
                    !=
                    tile2.get_rules()(tile1, direction.opposite())
                {
                    println!("ERROR: Rules are not bidirectional for {tile1:?} {tile2:?} {direction:?}")
                }
            });
        // TODO: Check if all directions have an opposite. And that that opposite truly is the opposite.
    }

    /// Returns the board to its empty state. Exactly the same as Board::new(), except wihout creating a new object
    pub fn clean(&mut self) {
        self.tiles = vec![];
        for cur_layer in 0..self.height as usize {
            let mut layer = vec![];
            for _ in 0..self.length {
                let mut row = vec![];
                for _ in 0..self.width {
                    row.push(MaybeTile::Undecided(T::possibles(cur_layer)))
                }
                layer.push(row)
            }
            self.tiles.push(layer)
        }
        self.decision_stack = vec![];
        self.dead_ends = vec![];
        self.current_layer = 0;
    }

    /// Fill the entire board.
    /// Either it retuns Ok(()) and the board is full, or the board is impossible to fill. See [ImpossibleBoardError]
    pub fn generate(&mut self) -> Result<(), ImpossibleBoardError> {
        let mut complete = false;
        while !complete {
            complete = self.generate_1()?;
        };
        Ok(())
    }

    /// Generate a single tile. It may generate more than one if the generated tile makes it so that only 1 tile can be in another position.
    /// returns true if the board has been filled. False if not. [ImpossibleBoardError] if it can't continue.
    pub fn generate_1(&mut self) -> Result<bool, ImpossibleBoardError> {
        match self.get_status() {
            BranchStatus::Complete => Ok(true),
            BranchStatus::DeadEnd => {self.go_back()?;Ok(false)},
            BranchStatus::Incomplete => {
                let (tile, row, col, layer) = if self.can_continue_branch() {
                    if let Some(current_branch) = self.decision_stack.last_mut() {
                        let (row, col, layer) = current_branch.deciding_coord;
                        let mut rng = rand::thread_rng();
                        let mut weights = Vec::new(); // TODO: Take weights into account
                        let options = match &self.tiles[layer][row][col] {
                            MaybeTile::Undecided(possibilities) => possibilities
                                .iter()
                                .filter(|tile| !current_branch.tried_tiles.contains(tile))
                                .map(|tile| {
                                    weights.push(tile.get_distribution(layer));
                                    tile
                                })
                                .collect::<Vec<_>>(),
                            MaybeTile::Decided(_) => unreachable!(),
                        };
                        let dist = WeightedIndex::new(&weights).unwrap();
                        let choice = *options[dist.sample(&mut rng)];
                        current_branch.tried_tiles.insert(choice);
                        (choice, row, col, layer)
                    } else {
                        unreachable!()
                    }
                } else {
                    let (row, col, layer) = self.get_undecided();
                    let tile = self.make_decision(row, col, layer);
                    let new_branch = DecisionBranch::new(row, col, layer);
                    self.decision_stack.push(new_branch);
                    (tile, row, col, layer)
                };
                self.change_tile(tile, row, col, layer);
                Ok(false)
            }
            BranchStatus::CompleteLayer => {self.current_layer += 1; Ok(false)},
        }
    }

    fn change_tile(&mut self, tile: T, row: usize, col: usize, layer: usize) {
        self.tiles[layer][row][col] = MaybeTile::Decided(tile);
        let new_propagated = self.propagate(tile, row, col, layer);
        let current_branch = self.decision_stack.last_mut().unwrap();
        current_branch.tried_tiles.insert(tile);
        current_branch.temp_decided_coords.extend(new_propagated.into_iter());
    }

    /// Tries to set a tile. This can only be done to reduce possibilities, otherwise it will return [BadPlacementError].
    /// ## example
    /// Assuming our tile is:
    /// ```
    /// enum MyTile {
    ///     Yellow,
    ///     Green,
    ///     Red
    /// }
    /// ```
    /// This code will panic:
    /// ```should_panic
    #[doc = include_str!("../doc_helpers/MyTile.rs")]
    /// let mut board = procedural::Board::<MyTile>::new(5, 5, 1);
    /// # let mut yellow_and_green_and_red = std::collections::HashSet::new();
    /// # yellow_and_green_and_red.insert(MyTile::Yellow);
    /// # yellow_and_green_and_red.insert(MyTile::Green);
    /// # yellow_and_green_and_red.insert(MyTile::Green);
    /// // yellow_and_green_and_red is a hashmap containing Yellow, Green and Red.
    /// // On creation, every tile of the board is yellow_and_green_and_red. Since those are all the options
    /// assert_eq!(board.get_tile(0,0,0).unwrap(), procedural::MaybeTile::Undecided(yellow_and_green_and_red));
    /// 
    /// # let mut yellow_and_green = std::collections::HashSet::new();
    /// # yellow_and_green.insert(MyTile::Yellow);
    /// # yellow_and_green.insert(MyTile::Green);
    ///  // yellow_and_green is a HashSet that only contains Yellow and Green.
    /// board.set_tile(procedural::MaybeTile::Undecided(yellow_and_green), 0, 0, 0).unwrap(); // This won't panic.
    /// // Since 0,0,0 was never set. So both yellow and green are valid
    /// # let mut yellow_and_red = std::collections::HashSet::new();
    /// # yellow_and_red.insert(MyTile::Yellow);
    /// # yellow_and_red.insert(MyTile::Green);
    /// 
    ///  // yellow_and_red is a HashSet that only contains Yellow and Red
    /// board.set_tile(procedural::MaybeTile::Undecided(yellow_and_red), 0, 0, 0).unwrap(); // This will panic.
    /// // Since 0,0,0 doesn't have red as a possibility (due to the previous iine), and we tried to add it.
    /// ```
    pub fn set_tile(&mut self, tile: MaybeTile<T>, row: usize, col: usize, layer: usize) -> Result<(), BadPlacementError> {
        match tile {
            MaybeTile::Undecided(options) => match &self.tiles[layer][row][col] {
                MaybeTile::Undecided(possibilities) => if possibilities.is_superset(&options) {
                    self.tiles[layer][row][col] = MaybeTile::Undecided(options.clone());
                    let changes = self.propagate_possibilities(&options, row, col, layer);
                    let mut v = vec![];
                    for ((row, col, layer), new_possibilities) in changes {
                        match &mut self.tiles[layer][row][col] {
                            MaybeTile::Undecided(_) => {
                                self.tiles[layer][row][col] = MaybeTile::Undecided(new_possibilities);
                                v.push((row, col, layer))
                            },
                            MaybeTile::Decided(t) => {
                                if !new_possibilities.contains(t) {
                                    unreachable!()
                                }
                            },
                        }
                    }
                    let new_branch = DecisionBranch::new(row, col, layer);
                    self.decision_stack.push(new_branch);
                    let current_branch = self.decision_stack.last_mut().unwrap();
                    current_branch.temp_decided_coords.extend(v.into_iter());
                    Ok(())
                } else {
                    Err(BadPlacementError::NotAllPossible)
                },
                MaybeTile::Decided(tile) => if options.contains(tile) {
                    Err(BadPlacementError::TileAlreadyPlaced)
                } else {
                    Err(BadPlacementError::TileOccupied)
                },
            },
            MaybeTile::Decided(tile) => match &self.tiles[layer][row][col] {
                MaybeTile::Undecided(possibilities) => if possibilities.contains(&tile) {
                    let new_branch = DecisionBranch::new(row, col, layer);
                    self.decision_stack.push(new_branch);
                    self.change_tile(tile, row, col, layer);
                    Ok(())
                } else {
                    Err(BadPlacementError::ImpossibleTile)
                },
                MaybeTile::Decided(old_tile) => if tile == *old_tile {
                    Err(BadPlacementError::TileAlreadyPlaced)
                } else {
                    Err(BadPlacementError::TileOccupied)
                },
            },
        }
    }

    /// Returns the tile at a specified position
    pub fn get_tile(&self, row: usize, col: usize, layer: usize) -> Result<MaybeTile<T>, OutOfBoardError> {
        match self.tiles.get(layer) {
            Some(r) => match r.get(row) {
                Some(c) => match c.get(col) {
                    Some(t) => Ok((*t).clone()),
                    None => Err(OutOfBoardError),
                },
                None => Err(OutOfBoardError),
            },
            None => Err(OutOfBoardError),
        }
    }

    /// Generate n tiles at once
    pub fn generate_n(&mut self, n: u32) -> Result<bool, ImpossibleBoardError> {
        let mut complete = false;
        for _ in 0..n {
            if self.generate_1()? {
                complete = true;
                break
            }
        };
        Ok(complete)
    }

    /// Returns the [BranchStatus] of the current branch of the decision tree
    pub fn get_status(&self) -> BranchStatus {
        let undecideds_left = self.tiles.get(self.current_layer).unwrap().iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(_) => true,
            MaybeTile::Decided(_) => false,
        }));

        let impossibilities = self.tiles.get(self.current_layer).unwrap().iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(possibilities) => possibilities.len() == 0,
            MaybeTile::Decided(_) => false,
        }));

        let branch_end = self.tiles.get(self.current_layer).unwrap().iter().enumerate().all(|(i, v)| v.iter().enumerate().all(|(j, t)| match t {
            MaybeTile::Undecided(possibilities) => {
                if possibilities.len() > 0 {
                    if let Some(current_branch) = self.decision_stack.last() {
                        if current_branch.deciding_coord == (i, j, self.current_layer) {
                            if possibilities.len() <= current_branch.tried_tiles.len() {
                                true
                            } else {
                                false
                            }
                        } else if current_branch.dead_ends.contains(&(i, j, self.current_layer)) {
                            true
                        } else {
                            false
                        }
                    } else {
                        if self.dead_ends.contains(&(i, j, self.current_layer)) {
                            true
                        } else {
                            false
                        }
                    }
                } else {
                    true
                }
            },
            MaybeTile::Decided(_) => true,
        }));

        match (impossibilities, branch_end, undecideds_left) {
            (_, _, false) => match self.tiles.get(self.current_layer+1) {
                Some(_) => BranchStatus::CompleteLayer,
                None => BranchStatus::Complete,
            },
            (true, _, _) => BranchStatus::DeadEnd,
            (_, true, _) => BranchStatus::DeadEnd,
            (_, _, _) => BranchStatus::Incomplete,
        }
    }

    fn get_undecided(&mut self) -> (usize, usize, usize) {
        let options: Vec<_>;
        loop {
            let layer = self.tiles.get(self.current_layer).unwrap();
            let opt = layer.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    let board = &self;
                    row.iter()
                        .enumerate()
                        .filter_map(move |(j, tile)| match tile {
                            MaybeTile::Undecided(a) => if a.len() == 0 {
                                None
                            } else if let Some(current_branch) = board.decision_stack.last() {
                                if current_branch.dead_ends.contains(&(i, j, board.current_layer)) {
                                    None
                                } else {
                                    Some(((i, j, board.current_layer), a.len()))
                                }
                            } else {
                                if board.dead_ends.contains(&(i, j, board.current_layer)) {
                                    None
                                } else {
                                    Some(((i, j, board.current_layer), a.len()))    
                                }
                            },
                            MaybeTile::Decided(_) => None,
                        })
                }).collect::<Vec<_>>();
                let min = match opt.iter().min_by_key(|(_, key)| key) {
                    Some((_, min)) => Some(*min),
                    None => None,
                };
                match min {
                    Some(min) => {
                        options = opt.into_iter().filter(|(_, a)| *a == min).collect();
                        break;
                    },
                    None => self.current_layer += 1, // This layer is done
                }
        }
        let mut rng = rand::thread_rng();
        let option = options.iter().choose(&mut rng).unwrap().0;
        option
    }

    fn make_decision(&mut self, row: usize, col: usize, layer: usize) -> T {
        let mut rng = rand::thread_rng();
        let mut weights = Vec::new();
        let options = match &self.tiles[layer][row][col] {
            MaybeTile::Undecided(possibilities) => possibilities.iter()
                .map(|tile| {
                    weights.push(tile.get_distribution(layer));
                    tile
                }).collect::<Vec<_>>(),
            MaybeTile::Decided(_) => unreachable!(),
        };
        let dist = WeightedIndex::new(&weights).unwrap();
        *options[dist.sample(&mut rng)]
    }

    fn propagate(&mut self, tile: T, row: usize, col: usize, layer: usize) -> Vec<(usize, usize, usize)> {
        let mut v = vec![];
        let mut prop_dir = |direction: T::Direction, v: &mut Vec<(usize, usize, usize)>| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => {
                    match &mut self.tiles[layer][row][col] {
                        MaybeTile::Undecided(possibilities) => {
                            v.push((row, col, layer));
                            let len = possibilities.len();
                            tile.propagate(possibilities, direction);
                            if possibilities.len() == 1 {
                                let remaining_tile = *possibilities.iter().next().unwrap();
                                self.tiles[layer][row][col] = MaybeTile::Decided(remaining_tile);
                                v.extend(self.propagate(remaining_tile, row, col, layer).into_iter());
                            } else if possibilities.len() < len {
                                let possibilities = possibilities.clone();
                                let changes = self.propagate_possibilities(&possibilities, row, col, layer);
                                for ((row, col, layer), new_possibilities) in changes {
                                    match &mut self.tiles[layer][row][col] {
                                        MaybeTile::Undecided(_) => {
                                            self.tiles[layer][row][col] = MaybeTile::Undecided(new_possibilities);
                                            v.push((row, col, layer))
                                        },
                                        MaybeTile::Decided(t) => {
                                            if !new_possibilities.contains(t) {
                                                unreachable!()
                                            }
                                        },
                                    }
                                }
                            }
                        },
                        MaybeTile::Decided(_) => (),
                    }
                },
                Err(_) => ()
            }
        };
        for direction in Direction::all() {
            prop_dir(direction, &mut v);
        }
        v
    }

    fn propagate_possibilities(&self, possibilities: &HashSet<T>, row: usize, col: usize, layer: usize) -> Vec<((usize, usize, usize), HashSet<T>)> {
        let mut v = vec![];
        let prop_dir = |tile: T, direction: T::Direction| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => {
                    match &self.tiles[layer][row][col] {
                        MaybeTile::Undecided(possibilities) => {
                            let mut new_possibilities = possibilities.clone();
                            tile.propagate(&mut new_possibilities, direction);

                            Some(new_possibilities)
                        },
                        MaybeTile::Decided(_) => None,
                    }
                },
                Err(_) => None
            }
        };
        for direction in T::Direction::all() {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => match &self.tiles[layer][row][col] {
                    MaybeTile::Undecided(next_possibilities) => {
                        let len = next_possibilities.len();
                        let mut next_possibilities = HashSet::new();
                        for tile in possibilities.iter() {
                            match prop_dir(*tile, direction) {
                                Some(h) => for t in h {
                                    next_possibilities.insert(t);
                                },
                                None => (),
                            };
                        }
                        if len != next_possibilities.len() {
                            v.push(((row, col, layer), next_possibilities))
                        }
                    },
                    MaybeTile::Decided(_) => (),
                },
                Err(_) => (),
            }
        }
        v
    }

    #[cfg(feature = "view3d")]
    /// Draws the current state of the board
    pub fn draw(&self, gpu: &GpuState, te_state: &mut TeState) {
        for (k, layer) in self.tiles.iter().enumerate() {
            for (i, row) in layer.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        MaybeTile::Undecided(_) => (),
                        MaybeTile::Decided(tile) => {
                            if tile.has_model() {
                                te_state.place_custom_model(&tile.get_name(), gpu, (j as f32,k as f32,i as f32), None);
                            }
                        },
                    }
                }
            }
        }
    }

    #[cfg(feature = "view3d")]
    /// Loads all models so they can be rendered 
    pub fn load_models(&self, gpu: &GpuState, te_state: &mut TeState) {
        for tile in T::all() {
            match tile.get_model() {
                Some((vertices, indices, texture_name)) => {
                    let name = tile.get_name();
                    let model = get_model(gpu, te_state, name.clone(), vertices, indices, texture_name);
                    te_state.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));            
                },
                None => (),
            }
        }
    }

    fn go_back(&mut self) -> Result<(), ImpossibleBoardError> {
        if let Some(current_branch) = self.decision_stack.pop() {
            let (row, col, layer) = current_branch.deciding_coord;
            self.tiles[layer][row][col] = MaybeTile::Undecided(T::possibles(layer));
            for (row, col, layer) in current_branch.temp_decided_coords.iter() {
                self.tiles[*layer][*row][*col] = MaybeTile::Undecided(T::possibles(*layer));
            };
            self.recalculate(row, col, layer);
            for (row, col, layer) in current_branch.temp_decided_coords.iter() {
                self.recalculate(*row, *col, *layer);
            };
            if let Some(previous_branch) = self.decision_stack.last_mut() {
                previous_branch.dead_ends.push(current_branch.deciding_coord);
                self.current_layer = previous_branch.deciding_coord.2;
            } else{
                self.current_layer = 0;
                self.dead_ends.push(current_branch.deciding_coord)
            }
            Ok(())
        } else {
            Err(ImpossibleBoardError)
        }
    }

    fn can_continue_branch(&self) -> bool {
        if let Some(current_branch) = self.decision_stack.last() {
            let (row, col, layer) = current_branch.deciding_coord;
            match &self.tiles[layer][row][col] {
                MaybeTile::Undecided(_) => true,
                MaybeTile::Decided(_) => false,
            }
        } else {
            false
        }
    }

    fn recalculate(&mut self, row: usize, col: usize, layer: usize) {
        let mut recalc = |direction: T::Direction| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row_t, col_t, layer_t)) => {
                    match self.tiles[layer_t][row_t][col_t] {
                        MaybeTile::Undecided(_) => (),
                        MaybeTile::Decided(tile) => {
                            let possibilities = match &mut self.tiles[layer][row][col] {
                                MaybeTile::Undecided(possibilities) => possibilities,
                                MaybeTile::Decided(_) => unreachable!(),
                            };
                            tile.propagate(possibilities, direction.opposite());
                            if possibilities.len() == 1 {
                                self.tiles[layer_t][row_t][col_t] = MaybeTile::Decided(*possibilities.iter().next().unwrap())
                            }
                        },
                    }
                },
                Err(_) => ()
            }
        };
        for direction in Direction::all() {
            recalc(direction);
        }
    }
}

#[cfg(feature = "view3d")]
fn get_model(gpu: &GpuState, te_state: &mut TeState, name: String, vertices: Vec<ModelVertex>, indices: Vec<u32>, texture_name: String) -> te_renderer::model::Model {
    let image_path = Path::new("resources").join("tiles").join(texture_name);
    let img = image::open(image_path).unwrap();
    let img = img.as_rgba8().unwrap();
    let texture = te_renderer::texture::Texture::from_dyn_image(
        &gpu.device,
        &gpu.queue,
        &img,
        None
    ).unwrap();
    let materials = vec![te_renderer::model::Material::new(
        &gpu.device,
        "material",
        texture,
        &te_state.instances.layout
    )];
    let material_count = 0;
    let mesh = te_renderer::model::Mesh::new(
        name.clone(),
        &name,
        vertices,
        indices,
        material_count,
        &gpu.device
    );
    let meshes = vec![mesh];
    te_renderer::model::Model{ meshes, transparent_meshes: vec![], materials }
}

/// Returned when trying to access a position outside of the board
pub struct CoordError;

macro_rules! direction {
    () => {
        /// All the possible directions
        fn all() -> Vec<Self>;
        /// The position this direction points to from a certain position
        fn neighbour(&self, row: usize, col: usize, layer: usize, width: u32, length: u32, height: u32) -> Result<(usize, usize, usize), CoordError>;
        /// The direction that works exactly opposite when using [Direction::neighbour()]
        fn opposite(&self) -> Self;
    }
}

#[cfg(feature = "validate")]
/// Directions that are relevant to decide if a tile can be placed or not in a certain position
pub trait Direction: Sized + Copy + Debug {
    direction!();
}

#[cfg(not(feature = "validate"))]
/// Directions that are relevant to decide if a tile can be placed or not in a certain position
pub trait Direction: Sized + Copy {
    direction!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the various states that can have a slot in the board
pub enum MaybeTile<T>
where
    T: Tile,
{
    /// The slot has more than 1 valid option.
    Undecided(HashSet<T>),
    /// The slot has been allocated to a tile.
    Decided(T),
}

macro_rules! tile {
    () => {
        /// Directions that are relevant to this type of tile.
        type Direction: Direction;

        /// Set containing all the possible tiles.
        fn all() -> HashSet<Self>;

        /// Like [Tile::all()], but depending on the layer.
        fn possibles(layer: usize) -> HashSet<Self>;
        #[cfg(feature = "view3d")]
        /// **distinct** name, one for each tile with a different model.
        fn get_name(&self) -> String;
        #[cfg(feature = "view3d")]
        /// If the model has a model, returns its vertex and triangle indices
        fn get_model(&self) -> Option<(Vec<ModelVertex>, Vec<u32>, String)>;
        #[cfg(feature = "view3d")]
        /// If this tile has a model or not (is invisible)
        fn has_model(&self) -> bool;
        /// How the rest of tiles will react when this one is decided
        fn propagate(&self, possibilities: &mut HashSet<Self>, direction: Self::Direction) {
            let can_stay = self.get_rules();
            let mut to_remove = vec![];
            for possibility in possibilities.iter() {
                if !can_stay(possibility, direction) {
                    to_remove.push(*possibility);
                }
            }
            for rem in to_remove {
                possibilities.remove(&rem);
            }
        }
        /// Determines which tiles can be next to this one, depending on the direction
        fn get_rules(&self) -> Box<dyn Fn(&Self, Self::Direction) -> bool + '_>;
        /// Get the chance of this tile being chosen randomly, where 1 is the lowest chance and 2 is twice as likely as 1, etc.
        fn get_distribution(&self, layer: usize) -> u32;
    }
}

#[cfg(feature = "validate")]
/// Represents a tile of the board
pub trait Tile: Sized + Eq + PartialEq + Hash + Clone + Copy + Debug {
    tile!();
}

#[cfg(not(feature = "validate"))]
/// Represents a tile of the board
pub trait Tile: Sized + Eq + PartialEq + Hash + Clone + Copy {
    tile!();
}
