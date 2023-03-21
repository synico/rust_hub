fn main() {
    let f = 2.1_f32;
    let f: f32 = 2.1;
    let f = 2.1;
    let f = 2.1_f64;
    let f: f64 = 2.1;

    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
