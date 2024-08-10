use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    // guess number game
    // let mut count=0;
    // println!("wellcome to guess numbwr game");
    // let secret_number=rand::thread_rng().gen_range(1..=100);
    // loop {
        // println!("please enter the number you guessed");
        // let mut guess=String::new();
        // io::stdin().read_line(&mut guess).expect("cannot read message");
        // let guess:u32=match guess.trim().parse(){
            // Ok(num)=>num,
            // Err(_)=>{
    //             print!("please enter a valid number");
    //             continue;
    //         }
    //     };
    //     println!("your guessed is {}",guess);
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => {
    //             println!("too small");
    //             count=count+1;
    //         },
    //         Ordering::Greater =>{ 
    //         println!("too big");
    //         count=count+1;
    //         },
    //         Ordering::Equal =>{
    //             print!("You Win!");
    //             println!("you tried {} times.",count);
    //             break;
    //         }
    //     }
    // }



//calculator
    // println!("enter the operation,(+,-,*,/)");
    // let mut operation=String::new();
    // io::stdin().read_line(&mut operation).unwrap();

    // println!("enter first number");
    // let mut num1=String::new();
    // io::stdin().read_line(&mut num1).unwrap();
    // let num1:f64=num1.trim().parse().unwrap();

    // println!("enter second number");
    // let mut num2=String::new();
    // io::stdin().read_line(&mut num2).unwrap();
    // let num2:f64=num2.trim().parse().unwrap();
    
    // match operation.trim() {
    //     "+"=>print!("Result: {}",num1+num2),
    //     "-"=>print!("Result: {}",num1-num2),
    //     "*"=>print!("Result: {}",num1*num2),
    //     "/"=>{
    //         if num2!=0.0{
    //             print!("Result: {}",num1/num2);
    //         }else {
    //             print!("cannot divide by zero");
    //         }
    //     }
    //     _ => print!("invalid operation"),
    // }
    
    // let n=9;
    // print!("fibo of a number {} is {}",n,is_prime(n));
    // let n=434343532;
    // print!("sum of digit of number{} is{};",n,sum_of_digit(n));
    // let s="hello";
    // print!("reverse of string {} is : {}",s,reverse_digit(s));
    // even_odd(1031);

    // swapping function
    // let(mut a,mut b)=(10,30);
    // std::mem::swap(&mut a, &mut b);
    // print!("a={} b={}",a,b);

    // let arr = [1, 2, 3, 4, 5];
    // print!("maximum in arr is {}",find_max(&arr));

    // let arr = [1, 2, 3, 4, 5 ,-5];
    // print!("minimum in arr is {}",min_element(&arr));

}




// fn bubble_sort(arr: &mut [i32]) {
//     let len = arr.len();
//     for i in 0..len {
//         for j in 0..len-i-1 {
//             if arr[j] > arr[j + 1] {
//                 arr.swap(j, j + 1);
//             }
//         }
//     }
// }


// fn min_element(arr:&[i32])->i32{
//     let mut min=arr[0];
//     for &item in arr{
//         if min> item{
//             min=item;
//         }
//     }
//     min
// }


// fn find_max(arr: &[i32]) -> i32 {
//     let mut max = arr[0];
//     for &item in arr{
//         if item > max {
//             max = item;
//         }
//     }
//     max
// }



// fn even_odd(n:u32){
//     if n%2==0 {
//         print!("number is even {}",n);
//     }else{
//         print!("number is odd {}",n);
//     }
// }

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