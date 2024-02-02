fn main() {
    for a in 1..500 {
        for b in (a+1)..1000 {
            let c = 1000 - a - b;
            if c <= b {
                break;
            }

            if a * a + b * b == c * c {
                println!("a = {}, b = {}, c = {}", a, b, c);
                println!("a * b * c = {}", a * b * c);
                return;
            }
        }
    }
}
