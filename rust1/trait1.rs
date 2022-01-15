fn main() {
    trait Log {
        fn display_info(&self) {
            
        }
    }

    struct Person {
        name: String,
        age: u32,
        id: u32
    }

    impl Person {

        fn new() -> Person {
            Person {
                name: "default".to_string(),
                age: 0,
                id: 0
            }
        }
    }


    impl Log for Person {
        fn display_info(&self) {
            println!("{} ", self.name)
        }
    }


    let person = Person::new(String::from("trevor"), 1, 1);
    person.display_info();
}
