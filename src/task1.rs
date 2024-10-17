#[test]

/*

/* Fill in the blanks and Fix the errors */
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("__ months in a year.", 12);

    println!("Now __ will print!", Structure(3));
}
*/
fn main() {
    // Виводимо число місяців у році
    println!("{} months in a year.", 12);

    // Тепер структура буде виведена правильно
    println!("Now {:?} will print!", Structure(3));
}


use std::fmt;

// Визначаємо структуру
struct Structure(i32);

// Реалізуємо трейт fmt::Debug для нашої структури
impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}




/*
Реалізація трейтa fmt::Debug: Це дозволяє структурі Structure коректно відображатися, коли ви намагаєтеся її вивести. В методі fmt визначається формат, в якому буде виводитися ваша структура.
Форматування в println!:
{}: використовується для виведення звичайних значень.
{:?}: використовується для виведення значень, які реалізують трейт Debug.
*/