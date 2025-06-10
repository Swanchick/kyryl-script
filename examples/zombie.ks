let map = [
    "Z...",
    "#.#.",
    "..#.",
    "#..."
];

let directions: [[int]] = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0]
];

let steps = 10;

function display(map: [string]) {
    for line in map {
        println(line);
    }
}

function step(map: [string]): [string] {
    let clone_map = map;

    println(clone_map);
    
    return ["123123"];
}

function main() {    
    step(map);
}

main();

