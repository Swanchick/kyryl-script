function bubble_sort(numbers: [int]): [int] {
    show_local();
    
    
    for j in range(len(numbers)) {
        for i in range(len(numbers) - j - 1) {
            let n1 = numbers[i];
            let n2 = numbers[i + 1];

            println(n1);
            println("Before: ", numbers[i + 1]);
            if (n1 > n2) {
                println("CHANGED!");
                
                numbers[i] = n2;
                println("numbers[i] = ", numbers[i]);

                numbers[i + 1] = n1;
                println("numbers[i + 1] = ", numbers[i + 1]);
            }

            println("After: ", numbers[i + 1]);

            println("===========");
        }
    }

    return numbers;
}

function test(numbers: [int]) {
    println(numbers);

    let n1 = numbers[2];
    let n2 = numbers[3];
    println("Before: ", n2);

    numbers[3] = n1;

    println("After: ", n2);

    numbers[2] = n2;

    println(numbers);
}


let numbers = [2, 23, 3, 54, 76, 12, 67, 3];

test(numbers);

// println(numbers);


