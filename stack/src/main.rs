// FIFO stack type structure.
fn main() {
    println!("Hello, world!");
    // factorial_n(1000);
    factorial_eff(20, 1);
}

fn factorial_n(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * factorial_n(n - 1)
}

fn factorial_eff(n: u64, r: u64) -> u64 {
    if n <= 1 {
        return r;
    }
    factorial_eff(n - 1, n * r)
}
