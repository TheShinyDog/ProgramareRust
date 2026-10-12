fn add_chars_n(mut text: String, to_add: char, number: i32) -> String {
    for _ in 0..number {
        text.push(to_add);
    }
    text
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
