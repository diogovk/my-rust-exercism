
pub fn sum_of_squares(x: u64) -> u64 {
    (1..x+1).fold(0, |acc, num| num*num+acc) 
}

pub fn square_of_sum(x: u64) -> u64 {
    let sum = (1..x+1).fold(0, |acc, num| acc+num) ;
    sum * sum
}

pub fn difference(x: u64) -> u64{
    square_of_sum(x)-sum_of_squares(x)
}
