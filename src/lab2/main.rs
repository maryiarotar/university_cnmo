extern crate lab1;

use lab1::lab1::functions;

use std::io;
use std::io::Write;
use std::convert::TryFrom;


fn main() {

    let mut size: usize = 4;
    let mut matrix_A :Vec<Vec<f64>> = vec![];
    let mut matrix_B :Vec<f64> = vec![];

    enter_matrix(&mut size, &mut matrix_A, &mut matrix_B);

    let matrix_Q = matrix_Q(size, &matrix_A, &matrix_B );

    print_Q(size, &matrix_Q);

    let q : f64 = {
        let mut sums:Vec<f64> = Vec::new();
        for row in &matrix_Q {
            let row_sum = row.iter().take(size).map(|x| x.abs()).sum();
            sums.push(row_sum);
        }
        *sums.iter().max_by(|a, b| a.abs().total_cmp(&b.abs())).unwrap()
    };

    println!("___________q = {}", q);

    if q < 1.0 { //convergence verification

        println!("________________JACOBI______________");
        method_jacobi(q, 0.0001, size, &matrix_A, &matrix_Q);
        println!("________________GAUSS_ZEIDEL___________");
        method_gauss_zeidel(q, 0.0001, size, &matrix_A, &matrix_Q);

    } else {
        println!("Convergence condition is not met!");
    }



}



fn method_jacobi(q :f64, eps :f64, size :usize, matrix_A :&Vec<Vec<f64>>, matrix_Q :&Vec<Vec<f64>>) {
    let mut iteration: i32 = 0;
    let mut criterion: f64 = 1.0;
    let mut x_0: Vec<f64> = Vec::new();

    for i in 0..size {
        x_0.push(matrix_Q[i][size]); //x0_i = c_i
    }

    while criterion > eps {
        iteration += 1;
        print!("\n----iteration: {}-------------------------\n w = ", iteration);
        let mut x_next: Vec<f64> = Vec::new();

        for i in 0..size {
            let mut x_i = matrix_Q[i][size]; //x_i = c+i
            for j in 0..size {
                x_i = x_i + (matrix_Q[i][j] * x_0[j]);
            }
            x_next.push(x_i);
        }

        let norma_xi_min_x0: f64 = {
            let mut temp: Vec<f64> = Vec::new();
            for i in 0..size {
                temp.push((x_next[i] - x_0[i]).abs());
            }
            for el in &temp {print!("\t {:.5}  ", el);} //w = Ax - b
            * temp.iter().max_by(|a, b| a.total_cmp(b)).unwrap()
        };
        criterion = (q / (1.0 - q))* norma_xi_min_x0;
        x_0 = x_next.clone();
        print!("\n x = ");
        for el in &x_0 {print!("\t {:.5}  ", el);}
        println!("\ncriterion: {}", criterion, );
    }
}



fn method_gauss_zeidel(q :f64, eps :f64, size :usize, matrix_A :&Vec<Vec<f64>>, matrix_Q :&Vec<Vec<f64>>) {
    let mut iteration: i32 = 0;
    let mut criterion: f64 = 1.0;
    let mut x_0: Vec<f64> = Vec::new();

    for i in 0..size {
        x_0.push(matrix_Q[i][size]); //x0_i = c_i
    }

    while criterion > eps {
        iteration += 1;
        print!("\n----iteration: {}-------------------------\n w = ", iteration);
        let mut x_next: Vec<f64> = Vec::new();

        for i in 0..size {
            let mut x_i = matrix_Q[i][size]; //x_i = c+i
            for j in 0..size {

                if j>=i {
                    x_i = x_i + (matrix_Q[i][j] * x_0[j]);
                }
                else {
                    x_i = x_i + (matrix_Q[i][j] * x_next[j]);
                }
            }
            x_next.push(x_i);
        }

        let norma_xi_min_x0: f64 = {
            let mut temp: Vec<f64> = Vec::new();
            for i in 0..size {
                temp.push((x_next[i] - x_0[i]).abs());
            }
            for el in &temp {print!("\t {:.5}  ", el);} //w = Ax - b
            * temp.iter().max_by(|a, b| a.total_cmp(b)).unwrap()
        };
        criterion = (q / (1.0 - q))* norma_xi_min_x0;
        x_0 = x_next.clone();
        print!("\n x = ");
        for el in &x_0 {print!("\t {:.5}  ", el);}
        println!("\ncriterion: {}", criterion, );
    }
}





fn print_Q(size :usize, matrix_Q :&Vec<Vec<f64>>) {
    println!("--------------matrix Q--------------------       ---- c ----");
    for elem in matrix_Q{
        for j in 0..size {
            print!("{:.5}    ", elem[j]);
        }
        println!("   |  {:.5}", elem[size]);
    }
}



fn matrix_Q(size :usize, matrix_A :&Vec<Vec<f64>>, matrix_B :&Vec<f64>) -> Vec<Vec<f64>> {

    let mut matrix_Q :Vec<Vec<f64>> = Vec::new();

    for i in 0..size{

        let mut temp :Vec<f64> = Vec::new();

        for j in 0..size{

            let q = if i!=j { -(matrix_A[i][j]
                / matrix_A[i][i]) } else {0.0};
            temp.push(q);

        }
        let c = matrix_B[i] / matrix_A[i][i];
        temp.push(c);

        matrix_Q.push(temp);
    }
    matrix_Q
}




fn enter_matrix(size: &mut usize, matrix_A: &mut Vec<Vec<f64>>, matrix_B :&mut Vec<f64>) {

    let mut answer :String = Default::default();
    print!("Do yo want to enter matrix? [y,n]: ----> ");
    io::stdout().flush().expect("an error..");
    io::stdin().read_line(&mut answer).expect("an stdin error!");

    match answer.trim() == "n" {
        /* to enter matrix automatically */
        true => {
            *size = 4;
            *matrix_A = vec![vec![17.7, 0.3, 1.4, 0.9],
                             vec![0.3, 20.1, -0.8, -1.2],
                             vec![1.4, -0.8, 21.9, 0.8],
                             vec![0.9, -1.2, 0.8, 17.6]];
            *matrix_B = vec![11.2, -20.3, 14.4, 17.9];

        }
        /* to enter matrix manually */
        false => {
            println!("Enter a size for matrix A: ");
            *size = usize::try_from(functions::in_and_parse_number() as u32).unwrap();
            println!("Enter elements for matrix A: ");
            for i in 0..*size {

                let mut temp :Vec<f64> = Vec::new();

                for j in 0..*size {
                    temp.push(functions::in_and_parse_number());
                }
                matrix_A.push(temp);
            }
            println!("Enter elements for matrix B: ");
            for i in 0..*size {
                matrix_B.push(functions::in_and_parse_number());
            }
        }
    }
}


fn convergence_verification(size :usize, matrix_A :&Vec<Vec<f64>>) -> bool {

    for i in 0..size {
        let mut sum = 0.0;
        for j in 0..size {
            if i != j {
                sum += matrix_A[i][j];
            }
        }
        if matrix_A[i][i] <= sum {
            println!("convergence condition is not met!!!");
            return false;
        }
    }
    true
}
