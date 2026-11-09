use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
struct Persoana {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let cale = "fisier1.txt";
    let mut max_age = Persoana {
        name: String::from(""),
        phone: String::from(""),
        age: 0,
    };

    let mut min_age = Persoana {
        name: String::from(""),
        phone: String::from(""),
        age: u32::MAX,
    };

    match fs::read_to_string(cale) {
        Ok(continut) => {
            if continut.trim().is_empty() {
                eprintln!("fisierul '{}' dar este gol", cale);
                return;
            }
            let mut vector_persoane: Vec<Persoana> = Vec::new();
            for linie in continut.lines() {
                if linie.is_empty() {
                    continue;
                }

                match serde_json::from_str::<Persoana>(linie) {
                    Ok(persoana) => {
                        vector_persoane.push(persoana);
                    }
                    Err(e) => {
                        eprintln!("Eroare la parsarea JSON-ului : {}", e);
                    }
                }
                
            }
            for p in &vector_persoane {
                if p.age > max_age.age {
                    max_age = p.clone();
                }
                if p.age < min_age.age {
                    min_age = p.clone();
                }
            }
            println!(
                "Persoana cu age maxima: {} {} ani, nr telefon {}",
                max_age.name, max_age.age, max_age.phone
            );
            println!(
                "Persoana cu age minima: {} {} ani, nr telefon {}",
                min_age.name, min_age.age, min_age.phone
            );
        }

        Err(e) => {
            println!("Eroare la deschiderea fisierului '{}': {}", cale, e);
        }
    }
}

