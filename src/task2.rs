#[test]

/*
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };

    /* Make it output:
    Person {
        name: "Sunface",
        age: 18,
    }
    */
    println!("{:?}", person);
}
*/
fn main() {
    let person = Person {
        name: "Sunface".to_string(),
        age: 18,
    };

    // Виводимо структуру Person у форматі Debug
    println!("{:?}", person);
}
#[derive(Debug)] // Додаємо трейт Debug для структури
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}




/*
#[derive(Debug)]: Цей атрибут автоматично реалізує трейт Debug для вашої структури Person, що дозволяє використовувати форматування {:?} для виведення значень цієї структури.
println!("{:?}", person);: Використовується для виведення значення person у форматі, що реалізує Debug. Результат буде таким:
*/