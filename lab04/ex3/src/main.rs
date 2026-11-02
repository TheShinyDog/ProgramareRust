use std::fs;

fn dezabreviere(text: &str) -> String {
    let mut text_final = String::from("");
    let abrevieri = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    for cuvant in text.split_whitespace() {
        let mut inlocuitor = cuvant;
        for (abreviere, cuv) in abrevieri {
            if inlocuitor == abreviere {
                inlocuitor = cuv;
                break;
            }
        }
        if !text_final.is_empty() {
            text_final.push(' ');
        }
        text_final.push_str(inlocuitor);
    }

    text_final
}

fn main() {
    let cale = "fisier1.txt";

    match fs::read_to_string(cale) {
        Ok(continut) => {
            let text_modificat = dezabreviere(&continut);

            println!("Textul original: {}", continut);
            println!("Textul modificat: {}", text_modificat);
        }
        Err(e) => {
            println!("nu am putut citi fișierul {}: {}", cale, e);
        }
    }
}
