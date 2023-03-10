fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;

    println!("{}", r2);

    let _d1 = &s;
    let _d2 = &s;
    let _d3 = &mut s;
    
    println!("{}, {}, and {}", _d1, _d2, _d3);
}
