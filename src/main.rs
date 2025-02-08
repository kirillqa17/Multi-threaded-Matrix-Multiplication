use std::sync::{Arc, Mutex};
use std::thread;

fn multiply_matrix(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let stolbec_a = a[0].len();
    let stolbec_b = b[0].len();

    let result = Arc::new(Mutex::new(vec![vec![0; stolbec_b]; rows_a]));

    let mut handles = vec![];

    for i in 0..rows_a {
        let a_row = a[i].clone();
        let b = b.clone();
        let result_shared = Arc::clone(&result);

        let handle = thread::spawn(move || {
            for j in 0..stolbec_b {
                let mut sum = 0;
                for k in 0..stolbec_a {
                    sum += a_row[k] * b[k][j];
                }

                let mut result = result_shared.lock().unwrap();
                result[i][j] = sum;
            }
        });

        handles.push(handle);
    }



    for handle in handles {
        handle.join().unwrap();
    }
    let ans = result.lock().unwrap().clone();

    ans
}

fn main() {

    let a = vec![
        vec![3, 2],
        vec![1, -1],
    ];

    let b = vec![
        vec![1, 3],
        vec![-1, 4],
    ];

    let result = multiply_matrix(a, b);

    for row in result {
        println!("{:?}", row);
    }
}
