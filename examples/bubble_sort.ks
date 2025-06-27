function bubble_sort(numbers: [int]) {
    let number_length = len(numbers);
    println(range(number_length));

    
    for j in range(number_length) {
        for i in range(number_length - j - 1) {
            
            let n1 = numbers[i];
            
            println("numbers[i] = ", ref(numbers[i]));
            println("n1 = ", ref(n1));

            let n2 = numbers[i + 1];

            println("numbers[i + 1] = ", ref(numbers[i + 1]));
            println("n2 = ", ref(n2));

            println("Before: ", numbers);

            if n1 > n2 {
                numbers[i] = n2;
                numbers[i + 1] = n1;
            }

            println("After: ", numbers);
        }
    }
}

let numbers = [123, 23, 56, 34, 675, 94];
println(numbers);
bubble_sort(numbers);
println(numbers);