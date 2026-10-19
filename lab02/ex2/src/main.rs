fn add_chars_n(text: &mut String, to_add: char, number: i32) {
    for _ in 0..number {
        text.push(to_add);
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
