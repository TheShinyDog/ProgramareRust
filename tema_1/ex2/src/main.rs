fn is_coprime(mut x: i32, mut y: i32) -> bool {
    x = x.abs();
    y = y.abs();
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    x == 1
}

fn main() {
    for i in 0..101 {
        for j in 0..101 {
            if is_coprime(i, j) {
                println!("{i} si {j} sunt coprime");
            } else {
                println!("{i} si {j} nu sunt coprime");
            }
        }
    }
}
