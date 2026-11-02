use std::{fs, io};

fn main() -> io::Result<()> {
    let cale = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    //let cale = "hosts_test.txt";
    let continut = fs::read_to_string(cale)?;

    for linie in continut.lines() {
        let linie_curatata = linie.trim();
        if linie_curatata.starts_with('#') || linie_curatata.is_empty() {
            continue;
        }

        let mut coloane = linie_curatata.split_whitespace();
        if let (Some(ip), Some(hostname)) = (coloane.next(), coloane.next()) {
            println!("{} => {}", hostname, ip);
        }
    }
    Ok(())
}
