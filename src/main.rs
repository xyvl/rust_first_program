use std::io;

fn main() {
    let mut cat1_input = String::new();
    let mut cat2_input = String::new();

    println!("Введите первый катет");

    match io::stdin().read_line(&mut cat1_input) {
        Ok(_) => {}
        Err(e) => {
            println!("ОШИБКА ВВОДА: {}", e)
        }
    }
    
    println!("Введите второй катет");

    match io::stdin().read_line(&mut cat2_input) {
        Ok(_) => {}
        Err(e) => {
            println!("ОШИБКА ВВОДА: {}", e)
        }
    }


    let cat1: f64 = cat1_input.trim().parse().unwrap();
    let cat2: f64 = cat2_input.trim().parse().unwrap();
    println!("Гипотенуза равна: {}", pythagorean_theorem(cat1, cat2))
}

fn pythagorean_theorem(c1: f64, c2: f64) -> f64 {
    (c1 * c1 + c2 * c2).sqrt()
}
