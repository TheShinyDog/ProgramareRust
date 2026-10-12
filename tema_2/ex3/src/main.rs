fn my_i_to_string(integer: i32) -> String {
    let mut n: i32 = integer;

    if n == 0 {
        return String::from("0");
    }

    let mut digits = String::from("");
    let mut i = 0;
    while n > 0 {
        let cifra = (n % 10) as u8;
        let c = (cifra + b'0') as char;
        digits.push(c);
        i += 1;
        if i % 3 == 0 && n > 10 {
            digits.push('_');
        }
        n /= 10;
    }

    digits.chars().rev().collect()
}

fn my_f_to_string(f_value: f32, zecimale: i32) -> String {
    let parte_intreaga: i32 = f_value as i32;
    let mut result = my_i_to_string(parte_intreaga);

    if zecimale == 0 {
        return result;
    }

    result.push('.');

    let mut parte_fractionala = f_value - (parte_intreaga as f32);
    for _ in 0..zecimale {
        parte_fractionala *= 10.0;
        let digit = parte_fractionala as i32;
        let c = (digit as u8 + b'0') as char;
        result.push(c);
        parte_fractionala -= digit as f32;
    }
    result
}

fn add_space(text: &mut String, number: i32) {
    for _ in 0..number {
        text.push(' ');
    }
}
fn add_str(text: &mut String, to_add: String) {
    text.push_str(&to_add);
}
fn add_integer(text: &mut String, to_add: i32) {
    text.push_str(&my_i_to_string(to_add));
}

fn add_float(text: &mut String, to_add: f32, zecimale: i32) {
    text.push_str(&my_f_to_string(to_add, zecimale));
}
fn main() {
    //merge doar cu window width 156
    let mut s = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, String::from("I 💚"));
    add_space(&mut s, 152);
    add_str(&mut s, String::from("RUST."));
    add_space(&mut s, 115);
    add_str(&mut s, String::from("Most"));
    add_space(&mut s, 12);
    add_str(&mut s, String::from("crate"));
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 12);
    add_str(&mut s, String::from("and"));
    add_space(&mut s, 5);
    add_str(&mut s, String::from("latest"));
    add_space(&mut s, 9);
    add_str(&mut s, String::from("is"));
    add_space(&mut s, 86);
    add_str(&mut s, String::from("downloaded"));
    add_space(&mut s, 8);
    add_str(&mut s, String::from("has"));
    add_space(&mut s, 14);
    add_str(&mut s, String::from("downloads"));
    add_space(&mut s, 5);
    add_str(&mut s, String::from("the"));
    add_space(&mut s, 9);
    add_str(&mut s, String::from("version"));
    add_space(&mut s, 4);
    add_float(&mut s, 2.038, 3);
    add_str(&mut s, String::from("."));
    print!("{}", s);
}
