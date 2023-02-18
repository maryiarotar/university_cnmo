extern crate approx;
extern crate num_traits;

mod functions;
use functions::{in_and_parse_number, dichotomy, how_long_to_count};

fn main() {

    let a = in_and_parse_number();
    let b = in_and_parse_number();
    let e = in_and_parse_number();

    println!("a = {a}, b = {b}, e = {e}");

    let d :(f64, i8) = dichotomy(a, b, e );
    println!("метод дихотомии. Корень: {}, итераций: {}", d.0, d.1);




    how_long_to_count(dichotomy, a, b, e);




}

