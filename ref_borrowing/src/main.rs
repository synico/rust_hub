fn main() {
    let s1:String = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut sc = s1.clone();

    change(&mut sc);

    println!("The length of '{}' is {}.", sc, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
