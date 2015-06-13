fn main() {
    let mut nums: Vec<i64> = vec![];
    for i in 1..101 {
        nums.push(i);
    }
    
    let sum = |a, i| a + i;
    
    let square_of_sum: i64 = nums.iter()
                        .fold(0, &sum) // woo, function pointers
                        .pow(2);
                        
    let sum_of_squares: i64 = nums.iter()
                        .map(|i| i.pow(2)) 
                        .fold(0, |a, i| a + i); // no idea why 'sum' fails here
     
    println!("Difference is {}", square_of_sum - sum_of_squares);
                        
}
