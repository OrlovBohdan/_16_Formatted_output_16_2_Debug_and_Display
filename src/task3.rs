#[test]

/*

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}
*/
fn main() {
    // Тепер це буде виводити значення як потрібно
    println!("Now {} will print!", Deep(Structure(7)));
}
use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// Реалізація трейту Display для структури Deep
impl fmt::Display for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Виводимо лише значення з Structure
        write!(f, "{}", (self.0).0)
    }
}




/*
Трейт fmt::Display: Для структури Deep реалізується трейт fmt::Display, що дозволяє вам визначити, як буде виглядати вивід для об'єктів цієї структури.
Метод fmt: У методі fmt ви використовуєте write!, щоб вивести тільки значення i32 з вашої структури Structure. Ви можете отримати це значення за допомогою (self.0).0, де self.0 – це екземпляр Structure, а .0 – це доступ до його поля i32.
Вивід: Тепер при виклику println! з Deep(Structure(7)) буде виведено лише число 7.
*/