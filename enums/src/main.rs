#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
}

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name={} age={} children={}",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "Matt".to_string(),
        age: 35,
        children: 4,
    };
    println!("Hellow, people from: {:?}", p);
    let c = Color::Red("hello".to_string());
    match c {
        Color::Red(_) => println!("It's red"),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }

    let a = divide(10, 5);
    let b = divide(10, 0);
    match a {
        Ok(v) => println!("val={}", v),
        _ => {}
    }
    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}
