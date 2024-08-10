
fn main(){
    // let n=9;
    // print!("fibo of a number {} is {}",n,is_prime(n));
    // let n=434343532;
    // print!("sum of digit of number{} is{};",n,sum_of_digit(n));
    // let s="hello";
    // print!("reverse of string {} is : {}",s,reverse_digit(s));
}

// fn reverse_digit(s:&str)-> String{
//     return s.chars().rev().collect()
// }

// fn sum_of_digit(n:u32)-> u32{
//     return n.to_string().chars().map(|c|c.to_digit(10).unwrap()).sum();
// }

// fn is_prime(n:u32) -> bool {
//     if n<=1 {
//         return false;
//     } 
//     for i in 2..n{
//         if n%i==0 {
//             return false;
//         }
//     }
//     true
// }
// fn fibo(n:u32)->u32{
//     match n {
//         0=>0,
//         1=>1,
//         _=>fibo(n-1)+fibo(n-2),
//     }
// }