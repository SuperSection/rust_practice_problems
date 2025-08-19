/* Problem Question 2.
*
* Write a function `fib` that finds the fibonacci of a number
* that it takes as input
*/


fn main() {
    let num = 7;
    println!("Fibonacci of {} is: {}", num, fib(num));
}

// Fibonacci Series:
// 0 1 1 2 3 5 8 13 21 34 55 ...

fn fib(num: u32) -> u32 {
    if num == 0 || num == 1 {
        return num;
    }

    return fib(num - 1) + fib(num - 2);
}
