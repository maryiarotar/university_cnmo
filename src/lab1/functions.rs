
use std::io;
use std::io::Write;
use std::time::{Instant};

/*dichotomy method*/
pub(crate) fn dichotomy(a :f64, b :f64, precision :f64) -> (f64, i8) {
    let mut start = a;
    let mut end = b;
    let mut iteration :i8 = 0;

//    println!("МЕТОД ДИХОТОМИИ");

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
//        println!("Корень: {}, _ итераций: {} __ f(x)={}", (start + end) / 2.0, iteration, function_x((start + end) / 2.0));
    }
}

/*secand method*/
pub(crate) fn secant_method(a :f64, b :f64, precision :f64) -> (f64, i8) {
    let mut iteration :i8 = 0;
    let func_a = function_x(a);

    let min1 = if proizv_1(a).abs() < proizv_1(b).abs() {proizv_1(a).abs()} else {proizv_1(a).abs()};
    let max1 = if proizv_1(a).abs() > proizv_1(b).abs() {proizv_1(a).abs()} else {proizv_1(a).abs()};

//    println!("MAX1: {},   _ MIN1: {}", max1, min1);

    if func_a * proizv_2(a) > 0.0 {
        let mut x_cur :f64 = b;
        loop {
//            println!("Корень: {}, _ итераций: {} __ f(x)={}", x_cur, iteration, function_x(x_cur));
            iteration += 1;
            let func_x_cur = function_x(x_cur);
            let x_next = x_cur - ((x_cur - a) * func_x_cur)
                / (func_x_cur - func_a);
            if ((x_next - x_cur).abs() <= (precision*min1)/(max1-min1)) &&
                (function_x(x_next).abs() <= precision) {
                return (x_next, iteration);
            }
            x_cur = x_next;
        }

    } else { //if f(a)*f"(a)<0
        let func_b = function_x(b);
        let mut x_cur :f64 = a;
        loop {
//            println!("Корень: {}, _ итераций: {} __ f(x)={}", x_cur, iteration, function_x(x_cur));
            iteration += 1;
            let func_x_cur = function_x(x_cur);
            let x_next = x_cur - ((b - x_cur) * func_x_cur)
                / (func_b - func_x_cur);
            if ((x_next - x_cur).abs() <= (precision*min1)/(max1-min1)) &&
                (function_x(x_next).abs() <= precision) {
                return (x_next, iteration);
            }
            x_cur = x_next;
        }
    }
}

//Newton's_method
pub(crate) fn newton(a :f64, b :f64, precision :f64) -> (f64, i8) {
    let mut iteration :i8 = 0;

    let mut x_cur = if function_x(a)*proizv_2(a) > 0.0 {a}
        else {if function_x(b)*proizv_2(b) > 0.0 {b} else {0.0}};

    if x_cur == 0.0 {println!("Choose another [a, b]!"); return (0f64, 0i8);}

    let max2 = if proizv_2(a).abs() > proizv_2(b).abs() {proizv_2(a).abs()} else {proizv_2(a).abs()};
    let min1 = if proizv_1(a).abs() < proizv_1(b).abs() {proizv_1(a).abs()} else {proizv_1(a).abs()};

    loop {
//        println!("Корень: {}, _ итераций: {} __ f(x)={}", x_cur, iteration, function_x(x_cur));
        iteration += 1;
        let x_next :f64 = x_cur - function_x(x_cur)/proizv_1(x_cur);

         if (max2/(2.0*min1))*((x_next - x_cur)*(x_next - x_cur)) <= precision {
                return (x_next, iteration);
        }

        x_cur = x_next;
    }
}

//iteration_method
pub(crate) fn iteration_method(a :f64, b :f64, precision :f64) -> (f64, i8) {

    let mut iteration: i8 = 0;
    // q = 1-(m1/M1)
    let min1 = if proizv_1(a).abs() < proizv_1(b).abs() {proizv_1(a).abs()} else {proizv_1(b).abs()};
    let max1 = if proizv_1(a).abs() > proizv_1(b).abs() {proizv_1(a).abs()} else {proizv_1(b).abs()};
    let q = 1.0 - (min1/max1);
 //   println!("min1: {}, _ max1: {}", min1, max1);
 //   println!("(1-min1/max1) < 1 ____  = {} < 1", 1.0-(min1/max1));

    let max = if proizv_1(a).abs() > proizv_1(b).abs() { proizv_1(a).abs() } else { proizv_1(b).abs() };

    let mut x_cur = if function_x(a)*proizv_2(a) > 0.0 {a}
        else {if function_x(b)*proizv_2(b) > 0.0 {b} else {0.0}};

    let mut x_next :f64 = 0.0;
    
    loop {
 //       println!("Корень: {}, _ итераций: {} __ f(x)={}", x_cur, iteration, function_x(x_cur));
        iteration += 1;
        if proizv_1(x_cur) > 0.0 {
            x_next = x_cur - function_x(x_cur)*(1.0/max);
        }
        if proizv_1(x_cur) < 0.0 {
            x_next = x_cur - (function_x(x_cur)*-1.0)*(1.0/max);
        }

        if ((x_next - x_cur).abs()*(q/(1.0-q))) < precision{
            return (x_next, iteration);
        }
        x_cur = x_next;
    }

}

/* f(x) = 𝑥^2 𝑐𝑜𝑠2𝑥 + 1 */
fn function_x(x :f64) -> f64 {
    let func_x :f64 = (x*x) * (2.0*x).cos() + 1.0;
    func_x
}

fn proizv_1(x :f64) -> f64 {
    let proizv_1 :f64 = -2.0*(x*x)*(2.0*x).sin() + 2.0*x*(2.0*x).cos();
    proizv_1
}

fn proizv_2(x :f64) -> f64 {
    let proizv_2 :f64 = -4.0*(x*x)*((2.0*x).cos())
        - 8.0*x*((2.0*x).sin()) + 2.0*((2.0*x).cos());
    proizv_2
}

        /*function to enter a number*/

pub fn in_and_parse_number() -> f64 {
            let mut temp = String::new();
            loop {
                temp.clear();
                print!("enter a number: ---> ");
                io::stdout().flush().expect("an error..");

                io::stdin()
                    .read_line(&mut temp)
                    .expect("an stdin error!");

                let temp: f64 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                io::stdout().flush().expect("an error..");
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

