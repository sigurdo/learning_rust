use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    // 3.3: Funksjoner, statements og expressions
    fn hmmm(start: i32) -> i32 {
        let mut a = start;
        for _i in 0..12 {
            a *= 2;
        }
        a
    }

    let num = hmmm(1);

    println!("Hoho {}", num);

    // 3.4: Kontrollflyt
    let text = if num == 4096 { "Fett" } else { "Magert" };
    println!("{}", text);

    let text = match num {
        2 => "Bare 2",
        3 => "Oddetall, wtf?",
        2048 => "Nå er det 2048",
        _ => "Det var ingen av de lave",
    };
    println!("{}", text);

    // if true { let envariabel = "sant"; } else { 3 };

    let mut count_inner = 0;
    let mut count_outer = 0;
    'outer: loop {
        'inner: loop {
            println!("yo {}{}", count_outer, count_inner);
            count_inner += 1;
            match count_outer.cmp(&10) {
                Ordering::Greater | Ordering::Equal => break 'outer,
                _ => (),
            }
            match count_inner.cmp(&10) {
                Ordering::Greater | Ordering::Equal => {
                    count_inner = 0;
                    break 'inner;
                }
                _ => (),
            }
        }
        count_outer += 1;
    }

    let a = loop {
        break "håhå";
    };
    println!("a: {}", a);
}
