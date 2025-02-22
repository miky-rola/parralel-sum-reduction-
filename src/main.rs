use rayon::prelude::*;

fn parralel_sum(numbers: &[i32]) -> i32{

    if numbers.is_empty(){
        return  0;
    }
    numbers.par_iter().sum()
}

fn main(){
    
}