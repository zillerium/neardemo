 
fn abbrev_name(name: &str) -> String {
    // code away
    let   split = name.split(" ");
    println!("{:?} split ==> ", split);
    let vec: Vec<&str> = split.collect();

    println!("{:?} collect ==> ", vec);
    let mut a_string = String::new();
    let mut counter = 0;
    for i in vec {
        // i is a name - take first char
        let ch = i.chars().nth(0).unwrap();
        println!("{:?} ch ==>+  ", ch);
        a_string.push(ch);
        if counter == 0 {
             a_string.push('.');
             counter = counter + 1;
        }

        
    }
        
           println!("{:?} initials ==> ", a_string.to_uppercase());
     return "gg".to_string();

} 
    

fn main() {
 
   println!("{:?}", abbrev_name("john ross"));
 
}
