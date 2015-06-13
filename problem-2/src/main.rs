fn main() {
    let mut fibs: Vec<i64> = vec![2];
    let mut x = 1;
    let mut y = 2;
    let mut i: i64 = 0;
    
    while i < 4_000_000i64 {
        i = x + y;
        x = y;
        y = i;
        if i % 2 == 0 {
            fibs.push(i);
        }
    }
    
    println!("{}", fibs.iter().fold(0, |acc, item| acc + item));   
    
}


