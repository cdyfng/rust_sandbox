#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// struct Person {    // instead of: struct Person<'a> {
//     name: String,  // instead of: name: &'a str 
//     age: u8
// }

pub fn run() {
    let name = "Peter"; //String::from("Peter");//
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}