fn is_prime(x: i32) -> bool {
    if x < 1 {
        return false;
    }
    if x == 2 {
        return true;
    }
    for i in 2..=x / 2 {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for i in 0..101 {
        if is_prime(i) {
            println!("{i} este prim");
        } else {
            println!("{i} nu esre prim");
        }
    }
}
