fn main() {
    let mut palindromes = vec![0];
    
    for y in 99..1000 {
        for x in 99..1000 {
            let p = x * y;
            if is_palindromic(p) {
                palindromes.push(p);
            }
        }
    }
    
    match palindromes.iter().max() {
        Option::None => println!("No palindromes found"),
        Option::Some(x) => println!("The largest palindrome is: {}", x)
    };
 }

fn is_palindromic(i: i32) -> bool {
    let d = i % 10;
    let t = (i % 100) / 10;
    let h = (i % 1000) / 100;
    
    let h_ = (i % 10000) / 1000;
    let t_ = (i % 100000) / 10000;
    let mut d_ = (i % 1000000) / 100000;
    
    if i < 100000 {
        d_ = -1;
    } 
    
    if d == d_ && t == t_ && h == h_ {
        return true;
    }
    
    return false;
}