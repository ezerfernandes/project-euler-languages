/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two
-digit numbers is 9009 = 91 x 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    let mut max: u32 = 0;

    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if product < max {
                break;
            }
            if is_palindrome(product) {
                max = product;
                println!("{} x {} = {}", i, j, product);
            }
        }
    }

    println!("max: {}", max);
}

fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let rev = s.chars().rev().collect::<String>();
    s == rev
}
