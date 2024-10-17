#[test]

/*

/* Make it work */
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!");
}
*/
fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!");
}
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Отримуємо доступ до вектора.
        let vec = &self.0;

        write!(f, "[")?;

        // Ітеруємося по елементах вектора разом з їхніми індексами.
        for (count, v) in vec.iter().enumerate() {
            // Для всіх елементів, окрім першого, додаємо кому.
            if count != 0 { write!(f, ", ")?; }
            // Виводимо індекс та значення.
            write!(f, "{}: {}", count, v)?;
        }

        // Закриваємо дужку і повертаємо fmt::Result.
        write!(f, "]")
    }
}




/*
Форматування елементів:
У циклі, що перебирає елементи вектора, ми додаємо індекс елемента разом з його значенням у форматі count: value.
Вивід:
Тепер, коли ви запускаєте код, він виводитиме список у бажаному форматі [0: 1, 1: 2, 2: 3], а також проходитиме перевірку.
*/