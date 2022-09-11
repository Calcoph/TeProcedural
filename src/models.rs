use te_renderer::model::ModelVertex;

pub const SQUARE_V: [ModelVertex; 4] = [
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}
];

pub const SQUARE_I: [u32; 6] = [
    0, 1, 2,
    0, 2, 3
];

pub const HOUSE_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]}, // base
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}, // base
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // front wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // front wall
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // front wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // front wall
    
    ModelVertex { position: [0.40, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // door
    ModelVertex { position: [0.60, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // door
    ModelVertex { position: [0.60, 0.35, 0.9], tex_coords: [0.0, 0.0]}, // door
    ModelVertex { position: [0.40, 0.35, 0.9], tex_coords: [0.0, 0.0]}, // door
    
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // back wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.0, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [0.0, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // back wall
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // left wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // left wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // left wall
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [0.0, 0.0]}, // left wall

    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // right wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [0.0, 0.0]}, // right wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.0, 0.0]}, // right wall
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // right wall

    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // ceiling
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [0.0, 0.0]}, // ceiling
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // ceiling
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [0.0, 0.0]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.9], tex_coords: [0.0, 0.0]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.1], tex_coords: [0.0, 0.0]}, // ceiling
];

pub const HOUSE_I: &[u32] = &[
    0, 1, 2, // base
    0, 2, 3,

    //4, 5, 6, // front wall
    //4, 6, 7,
    
    //8, 9, 10, // door
    //8, 10, 11,

    // 7        6
    //   11  10
    // 4  8  9  5

    4, 8, 11, // front wall
    4, 11, 7,
    11, 6, 7,
    11, 10, 6,
    10, 5, 6,
    9, 5, 10,

    12, 13, 14, // back wall
    12, 14, 15,

    16, 17, 18, // left wall
    16, 18, 19,

    20, 21, 22, // right wall
    20, 22, 23,
    
    24, 28, 29, // ceiling
    24, 29, 27,

    28, 25, 26, // ceiling
    28, 26, 29,

    24, 25, 28, // ceiling
    27, 29, 26,
];

pub const MOUNTAIN_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.5, 2.5, 0.5], tex_coords: [0.0, 1.0]},
];

pub const MOUNTAIN_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    4, 5, 8,
    5, 6, 8,

    6, 7, 8,
    7, 4, 8,
];