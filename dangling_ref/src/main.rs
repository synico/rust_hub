fn main() {
    let reference_to_nothing = no_dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    
    &s // we return a reference to the String, s
} // Danger!

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}
