#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    // Box acts as a pointer to make the linkedList growable.
    next: Option<Box<LinkedList<T>>>,
}

// Require trait for T
impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}
fn main() {
    println!("Hello, world!");
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }
    println!("{:?}", ll);
    // Vec is a dinamically sized array
    let mut v: Vec<String> = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("hello".to_string());
    for i in 0..105 {
        v.push(i.to_string());
    }
    // println!("{:?}", v);
    println!("v.len = {}, capacity= {}", v.len(), v.capacity());
}
