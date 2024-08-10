
fn main(){
    let n=9;
    print!("fibo of a number {} is {}",n,is_prime(n));
}

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