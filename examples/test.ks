function callFunction(func: function(int, int): int): int {
    let result = func(10, 20);
    println("Works");
    return result;
}


function my_sum(a: int, b: int): int {
    return a + b;
}

println(callFunction(my_sum));
