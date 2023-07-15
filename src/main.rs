use std::io;

struct User {
    name: String,
    surname: String,
    age: usize,
}
fn main() {
    let mut user = User {
        name: String::new(),
        surname: String::new(),
        age: 0,
    };
    let mut check_ok: usize = 0;
    let mut debug = String::new();

    println!("Введите своё имя");
    match io::stdin().read_line(&mut user.name) {
        Ok(_) => {
            check_ok += 1;
        }
        Err(e) => {
            println!("Вы ввели неправильно своё имя - {}", e)
        }
    }

    println!("Введите свою фамилию");
    match io::stdin().read_line(&mut user.surname) {
        Ok(_) => {
            check_ok += 1;
        }
        Err(e) => {
            println!("Вы ввели неправильно свою фамилию - {}", e)
        }
    }

    println!("Введите свой возраст");
    match io::stdin().read_line(&mut debug) {
        Ok(_) => {
            (user.age, check_ok) = parse_string_in_usize(debug, check_ok);
        }
        Err(e) => {
            println!("Вы ввели неправильно свой возраст - {}", e)
        }
    }

    if check_ok == 3{
        println!(
            "Ваше имя - {}, ваша фамилия - {}, а возраст - {}",
            user.name.trim(),
            user.surname.trim(),
            user.age
        )
    }
}

fn parse_string_in_usize(el: String, check: usize) -> (usize, usize) {
    match el.trim().parse::<usize>() {
        Ok(_) => {
            return (el.trim().parse::<usize>().unwrap(), check + 1);
        }
        Err(e) => {
            println!("Вы ввели неправильно свой возраст - {}", e);
            return (0, check);
        }
    }
}
