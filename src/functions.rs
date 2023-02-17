use std::io;
use std::io::Write;

/*dichotomy method*/
pub(crate) fn dichotomy(start :f64, end :f64, precision :f64) -> f64 {

    let middle =

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
