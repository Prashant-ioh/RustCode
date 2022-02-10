// Program 1:Program to find the  area of a Rectangle
/*use std::io;
fn main(){
    let mut b=String::new();
    let mut h= String::new();

    println!("Enter the value of height and width:{},{}",b,h);

    io::stdin().read_line(&mut b).expect("failed to read line");
    io::stdin().read_line(&mut h).expect("failed to read line");
    let width:i32= b.trim().parse().expect("enter value is not integer");
    let height:i32=h.trim().parse().expect("enter value is not integer");

    let area= width*height;

 println!("area of Rectangle:{}",area);

} */

// Program 2:Simple interest

/*
use std::io;
fn main()
{
    let mut p =String::new();
    let mut r=String::new();
    let mut t=String::new();

    println!("Enter the value you want");

    io::stdin().read_line(&mut p).expect("failed to read line");
    io::stdin().read_line(&mut r).expect("failed to read line");
    io::stdin().read_line(&mut t).expect("failed to read line");

    let principle:i32=p.trim().parse().expect("enter value is not integer");
    let Amount:i32=r.trim().parse().expect("enter value is not integer");
    let Per_Annum:i32=t.trim().parse().expect("enter value is not integer");

    let Simple_interest=principle* Amount *Per_Annum;

    println!("Area of simple interest:{}",Simple_interest/100);

}

 */

// Program 3:  Program to find the surface area of the cylinder

use std::io;

fn main(){

    let mut r=String::new();
    let mut h=String::new();
    let mut pie=3.14;

    println!("")
    io::stdin().read_line(&mut r).expect("failed to read line");
    io::stdin().read_line(&mut h).expect("failed to read line");


}