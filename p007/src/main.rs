/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?
*/

fn main() {
    let mut primes = vec![2];
    let mut i = 3;
    while primes.len() < 10001 {
        if primes.iter().all(|&p| i % p != 0) {
            primes.push(i);
        }
        i += 2;
    }
    println!("{}", primes[10000]);
}
