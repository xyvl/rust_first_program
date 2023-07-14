use std::io;

fn main() {
    loop {
        let mut cat1_input = String::new();
        let mut cat2_input = String::new();
    
        let mut cat1= 0.0;
        let mut cat2= 0.0;
    
        println!("Введите первый катет");
    
        match io::stdin().read_line(&mut cat1_input) {
            Ok(_) => {}
            Err(e) => {
                println!("ОШИБКА ВВОДА: {}", e)
            }
        }
    
        match cat1_input.trim().parse::<f64>() {
            Ok(_) => {
                cat1 = cat1_input.trim().parse::<f64>().unwrap()
            }
            Err(_) => {
                println!("Вы ввели не число в первом катете.");
                continue;
            },
        }
    
        println!("Введите второй катет");
    
        match io::stdin().read_line(&mut cat2_input) {
            Ok(_) => {}
            Err(e) => {
                println!("ОШИБКА ВВОДА: {}", e)
            }
        }
    
        match cat2_input.trim().parse::<f64>() {
            Ok(_) => {
                cat2 = cat2_input.trim().parse::<f64>().unwrap()
            }
            Err(_) => {
                println!("Вы ввели не число во втором катете.");
                continue;
            },
        }
    
        println!("Гипотенуза равна: {}", pythagorean_theorem(cat1, cat2))
    }

}

fn pythagorean_theorem(c1: f64, c2: f64) -> f64 {
    (c1 * c1 + c2 * c2).sqrt()
}
