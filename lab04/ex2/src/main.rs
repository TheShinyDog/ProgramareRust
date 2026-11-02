use std::fs;
use std::time::Instant;

fn rot13(string: String) -> Result<String, String> {
    let mut text_final = String::from("");

    for c in string.chars() {
        if !c.is_ascii() {
            return Err("un caracter din fisier nu este ascii".to_string());
        }
        let rotated = match c {
            'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            _ => c,
        };
        text_final.push(rotated);
    }
    Ok(text_final)
}

fn main() {
    let cale = "fisier1.txt";
    let start = Instant::now();
    match fs::read_to_string(cale) {
        Ok(continut) => match rot13(continut) {
            Ok(rezultat) => {
                println!("ROT13: {}, timp {}", rezultat, start.elapsed().as_micros());
            }
            Err(eroare) => eprintln!("eroare: {}", eroare),
        },
        Err(e) => {
            println!("Nu am putut citi fișierul {}: {}", cale, e);
        }
    }
}
