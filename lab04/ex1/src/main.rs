use std::fs;

fn main() {
    let cale = "fisier1.txt";

    match fs::read_to_string(cale) {
        Ok(continut) => {
            let mut max_bytes = "";
            let mut max_char = "";

            for linie in continut.lines() {
                if max_bytes.len() < linie.len() {
                    max_bytes = linie;
                }
                if max_char.chars().count() < linie.chars().count() {
                    max_char = linie;
                }
            }

            println!(" linia cu cei mai multi bytes : {}", max_bytes);
            println!(" linia cu cele mai multe caractere : {}", max_char);
        }

        Err(eroare) => {
            println!("Eroare la deschiderea fisierului '{}': {}", cale, eroare);
        }
    }
}
