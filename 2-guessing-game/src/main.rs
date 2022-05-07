use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gjett tallet");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Vennligst skriv inn gjettet ditt");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Klarte ikke lese inn det du skrev");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Du gjetta {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Større"),
            Ordering::Greater => println!("Mindre"),
            Ordering::Equal => {
                println!("Riktig!");
                break;
            }
        }
    }
}
