fn main() {
    enum PersonId {
        Passport(String),
        IdentityCard(String)
    }

    struct Person {
        name: String,
        id: PersonId
    }

    impl Person {
        fn new(name: String, id: PersonId)->Person{
            Person {
                name,
                id
            }
        }
    }

   let  p =  Person::new("trevor".to_string(), PersonId::Passport("trevor oakley".to_string()));
   println!("{} {:?} ", p.name, p.id);
}
