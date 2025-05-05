class Node {
    constructor(x, y) {
        this.x = x;
        this.y = y;
        this.visited = false;
    }
}

let world = [];
for(let i = 0; i < 300; i++) {
    for(let j = 0; j < 300; j++) {
        world[i][j] = new Node(i, j);
    }
}

function gen_maze(world) {
    let initial_cell = world[0][0];

    
}