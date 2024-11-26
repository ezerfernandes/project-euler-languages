
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { current: 2, next: 3 }
}

fn main() {
    let mut result: u64 = 0;
    let iterator = fibonacci();
    for i in iterator.take_while(|x| *x < 4_000_000) {
        if i % 2 == 0 {
            result += i;
        }
    };
    println!("{}", result);
}
