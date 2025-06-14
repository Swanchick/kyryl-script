function test(some_shit: int) {
    some_shit?{
        println("Early return works!");
    };

    println("Some shit: ", some_shit);
}

test(123);

test(null);

