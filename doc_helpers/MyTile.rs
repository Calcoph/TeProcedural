# #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
# enum MyDirection {
# }
# impl procedural::Direction for MyDirection {
#     fn all() -> Vec<Self> {vec![]}
#     fn neighbour(&self, row: usize, col: usize, layer: usize, width: u32, length: u32, height: u32) -> Result<(usize, usize, usize), procedural::CoordError> {Ok((0,1,0))}
#     fn opposite(&self) -> Self {self.clone()}
# }
# #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
# enum MyTile {
#    Yellow,
#    Green,
#    Red
# }
# impl procedural::Tile for MyTile {
#       type Direction = MyDirection;
#       fn all() -> std::collections::HashSet<Self> {
#           let mut h = std::collections::HashSet::new();
#           h.insert(Self::Yellow);
#           h.insert(Self::Green);
#           h.insert(Self::Red);
#           h
#       }
#       fn possibles(layer: usize) -> std::collections::HashSet<Self> {
#           Self::all()
#       }
#       fn get_rules(&self) -> Box<dyn Fn(&Self, Self::Direction) -> bool + '_> {
#           Box::new(|_: &Self, _: Self::Direction| true)
#       }
#       fn get_distribution(&self, layer: usize) -> u32 {
#           1
#       }
# }
