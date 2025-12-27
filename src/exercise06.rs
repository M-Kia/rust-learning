trait Describable {
    fn describe(&self) -> String {
        String::from("(No description available)")
    }
}

struct Person {
    name: String,
    age: i8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person Name: {}\nPerson Age: {}", self.name, self.age)
    }
}

struct Car {
    make: String,
    model: String,
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("Car Make: {}\n Car Model: {}", self.make, self.model)
    }
}

struct Cat {}

impl Describable for Cat {}

fn print_description<T: Describable>(item: &T) -> () {
    println!("{}", item.describe());
}

fn main() {
    let person = Person {
        name: String::from("Mohammad-Hossein"),
        age: 24,
    };
    let car = Car {
        make: String::from("Benz"),
        model: String::from("230 W115"),
    };
    let cat = Cat {};

    print_description(&person);
    print_description(&car);
    print_description(&cat);
}
