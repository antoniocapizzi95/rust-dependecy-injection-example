trait Person {
    fn walk(&self);
}

struct PersonImpl {
    name: String
}

impl Person for PersonImpl {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

fn main() {
    let person: Box<dyn Person> = Box::new(PersonImpl{name: "john".to_string()});
    person.walk();
}