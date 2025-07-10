function curry(f: function(int, int): int): function(int, int): int {
    let a = 10;
    
    return function(a: int, b: int): int {
        return f(a, b);
    };
}

function sum(a: int, b: int): int {
    return a + b;
}

let result = curry(sum);
println(result(10, 20));