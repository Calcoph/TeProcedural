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

pub const HOUSE_N_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [0.5, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [0.5, 0.0]}, // base
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}, // base
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.5, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [1.0, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [1.0, 0.0]}, // front wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.5, 0.0]}, // front wall
    
    ModelVertex { position: [0.40, 0.0, 0.1], tex_coords: [0.5+0.3/1.6, 1.0]}, // door
    ModelVertex { position: [0.60, 0.0, 0.1], tex_coords: [0.5+0.5/1.6, 1.0]}, // door
    ModelVertex { position: [0.60, 0.35, 0.1], tex_coords: [0.5+0.5/1.6, 1.0-0.35/0.5]}, // door
    ModelVertex { position: [0.40, 0.35, 0.1], tex_coords: [0.5+0.3/1.6, 1.0-0.35/0.5]}, // door
    
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [1.0, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.5, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.5, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [1.0, 0.0]}, // back wall

    ModelVertex { position: [0.5, 1.0, 0.9], tex_coords: [0.75, 0.5]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.1], tex_coords: [0.75, 0.5]}, // ceiling
];

pub const HOUSE_N_I: &[u32] = &[
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

    4, 5, 6, // front wall
    4, 6, 7,

    13, 9, 10, // back wall
    13, 10, 14,
    10, 15, 14,
    10, 11, 15,
    11, 12, 15,
    8, 12, 11,

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

pub const HOUSE_E_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [0.5, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [0.5, 0.0]}, // base
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}, // base
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.5, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [1.0, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [1.0, 0.0]}, // front wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.5, 0.0]}, // front wall
    
    ModelVertex { position: [0.9, 0.0, 0.4], tex_coords: [0.5+0.3/1.6, 1.0]}, // door
    ModelVertex { position: [0.9, 0.0, 0.6], tex_coords: [0.5+0.5/1.6, 1.0]}, // door
    ModelVertex { position: [0.9, 0.35, 0.6], tex_coords: [0.5+0.5/1.6, 1.0-0.35/0.5]}, // door
    ModelVertex { position: [0.9, 0.35, 0.4], tex_coords: [0.5+0.3/1.6, 1.0-0.35/0.5]}, // door
    
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [1.0, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.5, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.5, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [1.0, 0.0]}, // back wall

    ModelVertex { position: [0.5, 1.0, 0.9], tex_coords: [0.75, 0.5]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.1], tex_coords: [0.75, 0.5]}, // ceiling
];

pub const HOUSE_E_I: &[u32] = &[
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

    4, 5, 6, // front wall
    4, 6, 7,

    13, 12, 15, // back wall
    13, 15, 14,
    
    12, 4, 7, // left wall
    12, 7, 15,
    
    5, 9, 10, // right wall
    5, 10, 6,
    10, 14, 6,
    10, 11, 14,
    11, 13, 14,
    8, 13, 11,
    
    7, 16, 17, // ceiling
    7, 17, 15,

    16, 6, 14, // ceiling
    16, 14, 17,

    7, 6, 16, // ceiling
    14, 15, 17,
];

pub const HOUSE_S_V: &[ModelVertex] = &[
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

pub const HOUSE_S_I: &[u32] = &[
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

pub const HOUSE_W_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [0.5, 1.0]}, // base
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [0.5, 0.0]}, // base
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]}, // base
    
    ModelVertex { position: [0.1, 0.0, 0.9], tex_coords: [0.5, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.0, 0.9], tex_coords: [1.0, 1.0]}, // front wall
    ModelVertex { position: [0.9, 0.5, 0.9], tex_coords: [1.0, 0.0]}, // front wall
    ModelVertex { position: [0.1, 0.5, 0.9], tex_coords: [0.5, 0.0]}, // front wall
    
    ModelVertex { position: [0.1, 0.0, 0.4], tex_coords: [0.5+0.3/1.6, 1.0]}, // door
    ModelVertex { position: [0.1, 0.0, 0.6], tex_coords: [0.5+0.5/1.6, 1.0]}, // door
    ModelVertex { position: [0.1, 0.35, 0.6], tex_coords: [0.5+0.5/1.6, 1.0-0.35/0.5]}, // door
    ModelVertex { position: [0.1, 0.35, 0.4], tex_coords: [0.5+0.3/1.6, 1.0-0.35/0.5]}, // door
    
    ModelVertex { position: [0.1, 0.0, 0.1], tex_coords: [1.0, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.0, 0.1], tex_coords: [0.5, 1.0]}, // back wall
    ModelVertex { position: [0.9, 0.5, 0.1], tex_coords: [0.5, 0.0]}, // back wall
    ModelVertex { position: [0.1, 0.5, 0.1], tex_coords: [1.0, 0.0]}, // back wall

    ModelVertex { position: [0.5, 1.0, 0.9], tex_coords: [0.75, 0.5]}, // ceiling
    ModelVertex { position: [0.5, 1.0, 0.1], tex_coords: [0.75, 0.5]}, // ceiling
];

pub const HOUSE_W_I: &[u32] = &[
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

    4, 5, 6, // front wall
    4, 6, 7,

    12, 8, 11, // left wall
    12, 11, 15,
    11, 7, 15,
    11, 10, 7,
    10, 4, 7,
    9, 4, 10,

    13, 12, 15, // back wall
    13, 15, 14,

    5, 13, 14, // right wall
    5, 14, 6,
    
    7, 16, 17, // ceiling
    7, 17, 15,

    16, 6, 14, // ceiling
    16, 14, 17,

    7, 6, 16, // ceiling
    14, 15, 17,
];

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
pub const MOUNTAIN_N_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
];

#[allow(unused)]
pub const MOUNTAIN_N_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    1, 2, 5,
    3, 0, 4,

    0,1,5,
    0,5,4,

    2,3,4,
    2,4,5
];

#[allow(unused)]
pub const MOUNTAIN_S_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    //ModelVertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
];

#[allow(unused)]
pub const MOUNTAIN_S_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    1, 2, 4,
    3, 0, 5,

    0,1,4,
    0,4,5,

    2,3,5,
    2,5,4
];

#[allow(unused)]
pub const MOUNTAIN_E_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
];

#[allow(unused)]
pub const MOUNTAIN_E_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    1, 2, 4,
    3, 0, 5,

    0,1,4,
    0,4,5,

    2,3,5,
    2,5,4
];

#[allow(unused)]
pub const MOUNTAIN_W_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    //ModelVertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
    //ModelVertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
];

#[allow(unused)]
pub const MOUNTAIN_W_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    1, 2, 4,
    3, 0, 5,

    0,1,4,
    0,4,5,

    2,3,5,
    2,5,4
];

#[allow(unused)]
pub const MOUNTAIN_D_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},
];

#[allow(unused)]
pub const MOUNTAIN_D_I: &[u32] = &[
    // 7   6
    //
    // 4   5

    // 3   2
    //
    // 0   1

    0, 2, 1, // bottom face
    0, 3, 2,

    1, 2, 6, // right face
    1, 6, 5,

    2, 3, 7, // back face
    2, 7, 6,

    3, 0, 4, // left face
    3, 4, 7,

    0, 1, 5, // front face
    0, 5, 4,

    4, 5, 6, // up face
    4, 6, 7,
];

#[allow(unused)]
pub const MOUNTAIN_U_V: &[ModelVertex] = &[
    ModelVertex { position: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0]}, 
    ModelVertex { position: [1.0, 0.0, 1.0], tex_coords: [1.0, 1.0]},
    ModelVertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0]},
    ModelVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0]},

    ModelVertex { position: [0.5, 1.0, 0.5], tex_coords: [0.0, 1.0]},
];

#[allow(unused)]
pub const MOUNTAIN_U_I: &[u32] = &[
    0, 1, 2,
    0, 2, 3,

    0, 1, 4,
    1, 2, 4,

    2, 3, 4,
    3, 0, 4,
];
