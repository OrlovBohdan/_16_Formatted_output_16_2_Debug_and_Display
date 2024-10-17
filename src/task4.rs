#[test]

/*

/* Make it work*/
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    /* Implement.. */
}

impl fmt::Debug for Point2D {
    /* Implement.. */
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");

    println!("Success!");
}
*/
fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Complex { real: 3.3, imag: 7.2 }");

    println!("Success!");
}

use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

// Реалізація трейту Display
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y) // Формат для виводу
    }
}

// Реалізація трейту Debug
impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.x, self.y) // Формат для виводу
    }
}



/*
Трейт fmt::Display:
У методі fmt ми використовуємо write!, щоб вивести координати x і y у форматі "x + yi", що є стандартним форматом для комплексних чисел.
Трейт fmt::Debug:
У методі fmt для Debug ми виводимо дані в форматі Complex { real: x, imag: y }, щоб відобразити дані структури в детальному вигляді.
*/