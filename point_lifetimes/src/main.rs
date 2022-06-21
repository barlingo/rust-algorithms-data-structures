#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person {
            name: name,
            age: age,
        }
    }
    // 4 options to call the print.
    // self and mut take ownership of the object and destroys it at the end.
    //              ro-own     ro-borrow    rw-borrow   rw-own
    // pub fn print(self       &self        &mut self   mut self)
    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }
    pub fn age_up(&mut self, n: i32) {
        self.age += n;
    }
    // self no longer exist after this function is run.
    pub fn dropme(self) {}
}

fn main() {
    println!("Hello, world!");
    let mut p = Person::new("matt".to_string(), 35);
    p.age_up(3);
    let s = p.greet();
    println!("{s}");
    let a = get_age(&p);
    p.age_up(2);
    println!("person's age is {}", a);
    let s2 = p.greet();
    println!("really: {}", s2);
}

pub fn get_age(s: &Person) -> &i32 {
    // How long will the pointer be valid???
    // need to bind the lifetime of incoming pointer to outgoing pointer
    &s.age
}
