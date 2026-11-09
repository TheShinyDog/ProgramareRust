use std::fs;
#[derive(Clone)]
struct Student {
    nume: String,
    nr_telefon: String,
    varsta: u32,
}

fn main() {
    let cale = "fisier1.txt";
    let mut max_varsta = Student {
        nume: String::from(""),
        nr_telefon: String::from(""),
        varsta: 0,
    };

    let mut min_varsta = Student {
        nume: String::from(""),
        nr_telefon: String::from(""),
        varsta: u32::MAX,
    };

    match fs::read_to_string(cale) {
        Ok(continut) => {
            if continut.trim().is_empty() {
                eprintln!("fisierul '{}' dar este gol", cale);
                return;
            }
            let mut vector_persoane: Vec<Student> = Vec::new();
            for (i, linie) in continut.lines().enumerate() {
                let mut date = linie.split(",");

                if let (Some(nume), Some(nr_telefon), Some(varsta)) =
                    (date.next(), date.next(), date.next())
                {
                    vector_persoane.push(Student {
                        nume: nume.to_string(),
                        nr_telefon: nr_telefon.to_string(),
                        varsta: varsta.trim().parse::<u32>().unwrap_or(0),
                    });
                } else {
                    eprintln!("Eroare la parsarea liniei {}: {}", i + 1, linie);
                }
            }
            for p in &vector_persoane {
                if p.varsta > max_varsta.varsta {
                    max_varsta = p.clone();
                }
                if p.varsta < min_varsta.varsta {
                    min_varsta = p.clone();
                }
            }
            println!(
                "Student cu varsta maxima: {} {} ani, nr telefon {}",
                max_varsta.nume, max_varsta.varsta, max_varsta.nr_telefon
            );
            println!(
                "Student cu varsta minima: {} {} ani, nr telefon {}",
                min_varsta.nume, min_varsta.varsta, min_varsta.nr_telefon
            );
        }

        Err(e) => {
            println!("Eroare la deschiderea fisierului '{}': {}", cale, e);
        }
    }
}
