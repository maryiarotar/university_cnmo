use std::io;
use std::io::Write;
use std::time::{Instant};
//use num_traits::AsPrimitive;

/*dichotomy method*/
pub(crate) fn dichotomy(a :f64, b :f64, precision :f64) -> (f64, i8) {
    let mut start = a;
    let mut end = b;
    let mut iteration :i8 = 0;

    loop {
        let middle = (start + end) / 2.0;
        let func_a = function_x(start);
        let func_c = function_x(middle);
        let func_b = function_x(end);
        iteration += 1;

        // f(a)*f(c)>0 => there is no root and we work with [c, b]. And vice versa.
        if (func_a * func_c) > 0.0 {
            start = middle;
        }
        if (func_c * func_b) > 0.0 {
            end = middle;
        }

        if func_c == 0.0 { return (middle, iteration); } //if c==0, it's the root!
        if (end - start) <= precision {
            return ((start + end) / 2.0, iteration);
        }
    }
}


/* f(x) = 𝑥^2 𝑐𝑜𝑠2𝑥 + 1 */
fn function_x(x :f64) -> f64 {
    let func_x :f64 = x.sqrt() * (2.0*x).cos() + 1.0;
    func_x
}

/*function to enter a number*/
pub(crate) fn in_and_parse_number() -> f64 {

    let mut temp = String::new();
    loop {

        temp.clear();
        print!("enter a number: ---> ");
        io::stdout().flush().expect("an error..");

        io::stdin()
            .read_line(&mut temp)
            .expect("an stdin error!");

        let temp :f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return temp;
    }
}

pub(crate) fn how_long_to_count(f :fn(f64, f64, f64) -> (f64, i8),
                     a :f64, b :f64, pr :f64) {
    let start = Instant::now();
    f(a, b , pr);
    let end = start.elapsed();
    println!("performance = {} seconds ({} nanosecs) ", end.as_secs_f64(), end.as_nanos());
}