function test(map: string) {
    let clone_map = map;

    clone_map = "asdasdasd";

    println("Map: ", map);
}

let map = "Hello";
test(map);
println("Map original: ", map);