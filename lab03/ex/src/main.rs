fn is_prime(n: u16) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    let n_u32 = n as u32;
    let mut i: u32 = 2;
    while i * i <= n_u32 {
        if n_u32.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        return None;
    }

    let mut aux = x + 1;
    loop {
        if is_prime(aux) {
            return Some(aux);
        }
        if aux != u16::MAX {
            aux += 1;
        } else {
            break;
        }
    }
    None
}

fn checked_add(x: u32, y: u32) -> u32 {
    let x_u64 = x as u64;
    let y_u64 = y as u64;
    let u32_max = u32::MAX as u64;

    if x_u64 + y_u64 > u32_max {
        panic!(" the addition succeedes u32 limits");
    }
    x + y
}

fn checked_mul(x: u32, y: u32) -> u32 {
    let x_u64 = x as u64;
    let y_u64 = y as u64;
    let u32_max = u32::MAX as u64;

    if x_u64 * y_u64 > u32_max {
        panic!(" the multiplication succeedes u32 limits");
    }
    x * y
}

enum MyError {
    U32ExceededAdd,
    U32ExceededMul,
}

fn checked_add_res(x: u32, y: u32) -> Result<u32, MyError> {
    let x_u64 = x as u64;
    let y_u64 = y as u64;
    let u32_max = u32::MAX as u64;

    if x_u64 + y_u64 > u32_max {
        return Err(MyError::U32ExceededAdd);
    }
    Ok(x + y)
}

fn checked_mul_res(x: u32, y: u32) -> Result<u32, MyError> {
    let x_u64 = x as u64;
    let y_u64 = y as u64;
    let u32_max = u32::MAX as u64;

    if x_u64 * y_u64 > u32_max {
        return Err(MyError::U32ExceededMul);
    }
    Ok(x * y)
}

enum CharError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    match c.to_uppercase().next() {
        Some(litera) => Ok(litera),
        None => Err(CharError::NotLetter),
    }
}
fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    match c.to_lowercase().next() {
        Some(litera) => Ok(litera),
        None => Err(CharError::NotLetter),
    }
}

fn print_char(c: char) -> Result<char, CharError> {
    if c.is_control() {
        return Err(CharError::NotPrintable);
    }
    println!("{}", c);
    Ok(c)
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }
    match c.to_digit(10) {
        Some(num) => Ok(num),
        None => Err(CharError::NotDigit),
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(CharError::NotBase16Digit);
    }
    match c.to_digit(16) {
        Some(num) => Ok(num),
        None => Err(CharError::NotDigit),
    }
}

fn print_error(error: CharError) {
    match error {
        CharError::NotAscii => println!("Error: Caracterul nu este ASCII."),
        CharError::NotDigit => println!("Error: Caracterul nu este cifra (0-9)"),
        CharError::NotBase16Digit => {
            println!("Error: caracterul nu este cifra hexazecimala (0-9, A-F)")
        }
        CharError::NotLetter => println!("Error: caracterul nu e litera"),
        CharError::NotPrintable => {
            println!("Error: caracterul nu este printabil ( este de control)")
        }
    }
}

fn impartire_fara_zero(a: u32, b: u32) -> Option<u32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn main() {
    let test_numbers: [u16; 7] = [0, 5, 14, 65500, 65520, 65521, u16::MAX];

    for &num in test_numbers.iter() {
        match next_prime(num) {
            Some(prime) => println!("The next prime after {:>5} is: {}", num, prime),
            None => println!("The next prime after {:>5} is: [None] (Exceeds u16)", num),
        }
    }

    let a: u32 = 1000;
    let b: u32 = 5000;

    println!("--- Testare cazuri valide ---");

    let suma = checked_add(a, b);
    println!("{} + {} = {}", a, b, suma);

    let produs = checked_mul(a, b);
    println!("{} * {} = {}", a, b, produs);

    // let overflow_add = checked_add_panic(max_val, 1);
    //let overflow_mul = checked_mul_panic(max_val, 2);

    let normal_a = 1000;
    let normal_b = 5000;
    let max_val = u32::MAX;

    match checked_add_res(normal_a, normal_b) {
        Ok(rezultat) => println!("{} + {} = {}", normal_a, normal_b, rezultat),
        Err(MyError::U32ExceededAdd) => println!("Eroare: Overflow la adunare"),
        _ => {}
    }

    match checked_mul_res(normal_a, normal_b) {
        Ok(rezultat) => println!("{} * {} = {}", normal_a, normal_b, rezultat),
        Err(MyError::U32ExceededMul) => println!("Eroare: Overflow la inmultire"),
        _ => {}
    }

    match checked_add_res(max_val, 1) {
        Ok(rezultat) => println!("Rezultat adunare: {}", rezultat),
        Err(MyError::U32ExceededAdd) => println!("adunarea a depasit limitele u32"),
        _ => {}
    }

    match checked_mul_res(max_val, 2) {
        Ok(rezultat) => println!("Rezultat inmultire: {}", rezultat),
        Err(MyError::U32ExceededMul) => println!("inmultirea a depasit limitele u32"),
        _ => {}
    }

    match to_lowercase('A') {
        Ok(rezultat) => println!("caracterul a devenit: {}", rezultat),
        Err(CharError::NotLetter) => println!("caracterul nu e litera"),
        _ => {}
    }

    match to_lowercase('?') {
        Ok(rezultat) => println!("caracterul a devenit: {}", rezultat),
        Err(CharError::NotLetter) => println!("caracterul nu e litera"),
        _ => {}
    }
    match to_uppercase('a') {
        Ok(rezultat) => println!("caracterul a devenit: {}", rezultat),
        Err(CharError::NotLetter) => println!("caracterul nu e litera"),
        _ => {}
    }
    match to_uppercase('}') {
        Ok(rezultat) => println!("caracterul a devenit: {}", rezultat),
        Err(CharError::NotLetter) => println!("caracterul nu e litera"),
        _ => {}
    }

    match print_char('a') {
        Ok(rezultat) => println!("caracterul {} s-a printat", rezultat),
        Err(CharError::NotPrintable) => println!("caracterul nu e printabil"),
        _ => {}
    }
    match print_char('\n') {
        Ok(rezultat) => println!("caracterul {} s-a printat", rezultat),
        Err(CharError::NotPrintable) => println!("caracterul nu e printabil"),
        _ => {}
    }
    match char_to_number('7') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(CharError::NotAscii) => println!("caracterul nu e ascii"),
        Err(CharError::NotDigit) => println!("caracterul nu e cifra"),
        _ => {}
    }
    match char_to_number('a') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(CharError::NotAscii) => println!("caracterul nu e ascii"),
        Err(CharError::NotDigit) => println!("caracterul nu e cifra"),
        _ => {}
    }
    match char_to_number('ă') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(CharError::NotAscii) => println!("caracterul nu e ascii"),
        Err(CharError::NotDigit) => println!("caracterul nu e cifra"),
        _ => {}
    }

    match char_to_number_hex('C') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(CharError::NotAscii) => println!("caracterul nu e ascii"),
        Err(CharError::NotBase16Digit) => println!("caracterul nu e cifra"),
        _ => {}
    }
    match char_to_number_hex('g') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('ă') {
        Ok(rezultat) => println!("caracterul s-a transformat in numarul : {}", rezultat),
        Err(e) => print_error(e),
    }

    let p5 = impartire_fara_zero(4, 5);
    let p5_err = impartire_fara_zero(4, 0);

    match p5 {
        Some(rezultat) => println!("rezultatul la 4/5 este {}", rezultat),
        None => println!("S-a impartit la 0"),
    }
    match p5_err {
        Some(rezultat) => println!("rezultatul este {}", rezultat),
        None => println!("S-a impartit la 0"),
    }
}
