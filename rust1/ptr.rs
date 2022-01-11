use std::ptr;

fn main() {

// Create some data, a raw pointer pointing to it and a null pointer
let data: u32 = 42;
let x = String::from("hello");
let raw_ptr = x.as_ptr() as *const u32;
let null_ptr = ptr::null() as *const u32;

// the {:p} mapping shows pointer values as hexadecimal memory addresses
println!("Data address: {:p}", &data);
println!("Raw pointer address: {:p}", raw_ptr); 
println!("Null pointer address: {:p}", null_ptr);
printstr(x);
}


fn printstr(s: String) {
println!("{}", s);

let raw_ptr = s.as_ptr() as *const u32;
println!("string address: {:p}",&s);
println!("Raw pointer address: {:p}", raw_ptr);


}
