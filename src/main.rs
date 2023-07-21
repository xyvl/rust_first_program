use std::io;

fn main() {
    println!("efe");
    let mut cat1 = 0.0;
    let mut cat2 = 0.0;

    loop {
        cat1 = input_number("Введите первый катет".to_string());
        if cat1 != 0.0 {
            break;
        }
    }
    loop {
        cat2 = input_number("Введите второй катет".to_string());
        if cat2 != 0.0 {
            break;
        }
    }

    println!("Гипотенуза равна: {}", pythagorean_theorem(cat1, cat2));
}

fn input_number(message: String) -> f64 {
    println!("{}", message);

    let mut text: String = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => {}
        Err(e) => {
            println!("ОШИБКА ВВОДА: {}", e)
        }
    }

    match text.trim().parse::<f64>() {
        Ok(_) => {
            return text.trim().parse::<f64>().unwrap();
        }
        Err(_) => {
            println!("Вы ввели не число в первом катете.");
            return 0.0;
        }
    }
}

fn pythagorean_theorem(c1: f64, c2: f64) -> f64 {
    (c1 * c1 + c2 * c2).sqrt()
}
