use std::io;

const FIB_FIRST: u32 = 0;
const FIB_SECOND: u32 = 1;

fn main() {
    println!("Hello, world!");
    
    let mut nth_num = String::new();
    io::stdin()
        .read_line(&mut nth_num)
        .expect("Not able to read the line");

    let nth_num : u32 = nth_num
        .trim()
        .parse()
        .expect("not a number");
    println!("nth numnbe is {}", fib(nth_num));
    
}

fn fibanocci(n: u32) {

    if n == 1 {
        println!("number is {}", FIB_FIRST);
        return;
    }else if n == 2{
        println!("number is {}", FIB_SECOND)
    }else{
        let mut count = 3;
        let mut prev_num = 1;
        let mut prev_prev_num = 0;
        let mut final_num:u32= 0;
        while count <= n {
                
            final_num = prev_num + prev_prev_num;
            prev_prev_num = prev_num;
            prev_num = final_num;
            count +=1;
            println!("numbers are {prev_prev_num}, {prev_num}, {final_num}");
        }
        println!("nthnumber is {final_num}");

    }

}

fn fib(n: u32) -> u32 {
    let mut t1: u32 =0;
    let mut t2: u32 = 1;
    let mut t3: u32 = 0;
    for int in 0..n {

        t3= t1+ t2;
        t1= t2;
        t2= t3;
    }
   return t3; 

}
