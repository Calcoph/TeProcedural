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
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [0.5, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [0.5, 0.0]}, // base
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}, // base
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.5, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [1.0, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [1.0, 0.0]}, // front wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.5, 0.0]}, // front wall
    
    ModelVertex { position: [0.40, 0.0, 0.9], tex_coords: [0.5+0.3/1.6, 1.0]}, // door
    ModelVertex { position: [0.60, 0.0, 0.9], tex_coords: [0.5+0.5/1.6, 1.0]}, // door
    ModelVertex { position: [0.60, 0.35, 0.9], tex_coords: [0.5+0.5/1.6, 1.0-0.35/0.5]}, // door
    ModelVertex { position: [0.40, 0.35, 0.9], tex_coords: [0.5+0.3/1.6, 1.0-0.35/0.5]}, // door
    
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [1.0, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.5, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.5, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [1.0, 0.0]}, // back wall

    ModelVertex { position: [0.5, 1.0, 0.9], tex_coords: [0.75, 0.5]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.1], tex_coords: [0.75, 0.5]}, // ceiling
];

pub const HOUSE_I: &[u32] = &[
    0, 1, 2, // base
    0, 2, 3,

    // back
    // 15      14
    //
    // 12      13

    // front
    // 7        6
    //   11  10
    // 4  8  9  5

    // ceiling
    // 15  17  14
    //
    // 7   16  6

    4, 8, 11, // front wall
    4, 11, 7,
    11, 6, 7,
    11, 10, 6,
    10, 5, 6,
    9, 5, 10,

    13, 12, 15, // back wall
    13, 15, 14,

    12, 4, 7, // left wall
    12, 7, 15,

    5, 13, 14, // right wall
    5, 14, 6,
    
    7, 16, 17, // ceiling
    7, 17, 15,

    16, 6, 14, // ceiling
    16, 14, 17,

    7, 6, 16, // ceiling
    14, 15, 17,
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

pub const TREE_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.25, 0.3, 0.75], tex_coords: [0.0, 0.0]}, 
    ModelVertex { position: [0.75, 0.3, 0.75], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.75, 0.3, 0.25], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.25, 0.3, 0.25], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.45, 0.3, 0.55], tex_coords: [0.0, 0.0]}, 
    ModelVertex { position: [0.55, 0.3, 0.55], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.55, 0.3, 0.45], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.45, 0.3, 0.45], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.45, 0.0, 0.55], tex_coords: [0.0, 0.0]}, 
    ModelVertex { position: [0.55, 0.0, 0.55], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.55, 0.0, 0.45], tex_coords: [0.0, 0.0]},
    ModelVertex { position: [0.45, 0.0, 0.45], tex_coords: [0.0, 0.0]},
];

pub const TREE_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    4, 5, 8,
    5, 6, 8,

    6, 7, 8,
    7, 4, 8,

    6, 7, 8,
    7, 4, 8,

    13, 14, 10, // front trunk
    13, 10, 9,

    16, 13, 9, // back trunk
    16, 9, 12,

    14, 16, 11, // right trunk
    14, 11, 10,

    15, 16, 12, // left trunk
    15, 12, 11
];
