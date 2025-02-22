use rayon::prelude::*;

fn parrelel_sum(numbers: &[i32]) -> i32 {
    if numbers.is_empty(){
        return 0;
    }

    numbers.par_iter().sum()
}

fn main(){
    let numbers: Vec<i32> = (1..200).collect();

    let seq_sum: i32 = numbers.iter().sum();

    let parellel_results = parrelel_sum(&numbers);

    println!("THIS IS THE SEQUENTIAL NUMBERS: {}", seq_sum);
    println!("THIS IS THE PARALLEL NUMBERS: {}", parellel_results);

    assert_eq!(seq_sum, parellel_results)
}