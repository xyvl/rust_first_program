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

    println!("Введите своё имя");
    loop {
        user.name = input_only_string_message(&"имя".to_string());
        if user.name != "0" {
            break;
        }
    }

    println!("Введите свою фамилию");
    loop {
        user.surname = input_only_string_message(&"фамилия".to_string());
        if user.surname != "0" {
            break;
        }
    }

    println!("Введите свой возраст");
    loop {
        user.age = parse_string_in_usize();
        if user.age != 404 {
            break;
        }
    }

    println!(
        "Ваше имя - {}, ваша фамилия - {}, а возраст - {}",
        user.name.trim(),
        user.surname.trim(),
        user.age
    )
}

fn input_only_string_message(t: &String) -> String {
    let text = input_text();
    let mut check = false;

    for ch in text.chars() {
        match ch.to_string().trim().parse::<usize>() {
            Ok(_) => {
                println!("Вы допустили ошибку при вводе, а именно ввели не допустимые символу, попробуйте снова");
                println!("Введите своё {}", &t);
                check = true;
            }
            Err(_) => {}
        }
    }
    if check {
        return "0".to_string();
    } else {
        return text;
    }
}

fn parse_string_in_usize() -> usize {
    let text = input_text();

    match text.trim().parse::<usize>() {
        Ok(_) => {
            return text.trim().parse::<usize>().unwrap();
        }
        Err(_) => {
            println!("Вы ввели свой возраст неправильно");
            return 404;
        }
    }
}

fn input_text() -> String {
    let mut text = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => {}
        Err(_) => {
            println!("Произошла ошибка при вводе -");
        }
    }
    return text;
}
