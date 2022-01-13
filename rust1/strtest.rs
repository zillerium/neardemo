fn main() {
    let mut message = String::from("hello link");
    let mut y1 = message.clone();
    let mut str = "hello".to_string();
    let sr = str.push_str("more");
    let slice = &message[2..4];
    println!("{}", slice);
    println!("{}", y1);
    let num2 = Box::new(100);
    println!("{}", num2);

    struct Person {
       name: String,
       last_name: String,
       age: u32
    }

    impl Person {
        fn some_function() {
            println!("print an instance");
        }
        fn change_name(&self) {
            println!("{} change", &self.name);
        }
    }

    let person = Person {
        name: "trevor".to_string(),
        last_name: "oakley".to_string(),
        age: 37,
    };
    Person::some_function();
    Person::some_function();
    person.change_name();

    println!("{} {} {} ", person.name, person.last_name, person.age);

}
