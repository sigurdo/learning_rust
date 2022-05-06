use std::io;

fn main() {
    println!("Gjett tallet");

    println!("Vennligst skriv inn gjettet ditt");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Klarte ikke lese inn det du skrev");

    println!("Du gjetta {}", guess);
}
