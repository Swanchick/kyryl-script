let map = [
    "Z...",
    "#.#.",
    "..#.",
    "#..."
];

let steps = 10;

function display(map: [string]) {
    for line in map {
        println(line);
    }
}

function step(): [string] {
    return ["123123"];
}

function main() {    
    for i in range(steps) {
        display(map);
    }
}


main();

