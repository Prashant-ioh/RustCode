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
/*
// Program 3:  Program to find the surface area of the cylinder
//Surface Area of Cylinder = 2 Π (r + h)
use std::io;


fn main(){

    let mut r=String::new();
    let mut h=String::new();
    let  pie=3.14;

    println!("Enter the radius and height");
    io::stdin().read_line(&mut r).expect("failed to read line");
    io::stdin().read_line(&mut h).expect("failed to read line");

    let radius:f32=r.trim().parse().expect("Enter wrong number");
    let height:f32=h.trim().parse().expect("Enter wrong number");

    let area= 2.0 * pie * (radius+height);

    println!("surface_area_of_cylinder;{}",area);
}

*/
/*
//Program 4: Program to Check Disarium number
//. Program to check Disarium Number = 1^1 + 7^2 + 5^3 = 1 + 49 + 125 = 175
use std::io;

fn main()
{
    println!("Enter the number");
    let mut number=String::new();

    io::stdin().read_line(&mut number).expect("failed to read number");

    let mut user_number=number.trim().parse().expect("Entered nuber is not correct");

    let mut sum=0;
    let mut rem;
    let n = user_number;

    let mut len=calculate_length(user_number);
    user_number = n.clone();
    println!("Length of the number:{}",len);
    while user_number > 0 {
        rem = user_number % 10;
        sum = sum + rem.pow(len as u32);
        println!("sum:{}",sum);
        user_number = user_number / 10;
        len = len-1;
    }
    if sum == n{
        println!("Entered number is disarium:{}", n);
    } else {
        println!("Entered number is not disarium");
    }

}

fn calculate_length(mut n:i32)->i32{
    let mut length=0;
    while n != 0{
        length=length+1;
        n=n/10;
    }
    length
}

/*
//. Program to check Disarium Number = 1^1 + 7^2 + 5^3 = 1 + 49 + 125 = 175
 */
use std::io;
fn main(){
    println!("Enter the number");
    let mut input_number =String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read the number");
    let mut number: i32 = input_number
        .trim()
        .parse()
        .expect("Entered value is not a number");
    let mut sum:i32 = 0;
    let mut rem;
    let mut len = calculate_length(number);
    let  input_number = number.clone();
    while number > 0 {
        rem = number % 10;
        sum = sum + rem.pow(len as u32);
        number = number / 10;
        len = len -1;
    }
    if sum == input_number {
        println!("Entered number:{} is a disarium number", input_number);
    } else {
        println!("Entered number is not a disarium number");
    }
}
fn calculate_length(mut number:i32) -> i32 {
    let mut length = 0;
    while number!=0{
        length = length + 1;
        number = number/ 10;
    }
    length
}

 */


/*
//program to Check Happy number.


use std::io;

fn ishappynumber( mut num:i32) -> i32
{
    let mut rem;
    let mut sum=0;

    while num > 0 {
        rem = num % 10;
        sum = sum + rem.pow(2);
        num = num / 10;
    }
    return  sum;
}

fn main()
{
    println!("Enter the  number");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let number=num.trim().parse().expect("Entered number is not correct");

    let mut result = number;

    while result!= 1 && result!= 4 {
        result = ishappynumber(result);
        println!("result is:{}",result);
    }

    if result == 1{
        println!("Happy number is:{}",number);
    }
    else {
        print!("Entered number is not happy number");
    }
}


//Program to print all Pronic numbers between 1 to 100 using loop

//use std::io;
fn main()
{
    let mut result;

    for i in  0..=100{
        result=sumofdigit(i);
        if result==i{
            println!("disarium number is:{}",i);
        }
}

}
    fn calculatelength(mut n:i32) -> i32 {

    let mut length =0;
    while  n != 0 {
        length = length + 1;
        n = n /10;

    }
    return length;
}

fn sumofdigit(mut num:i32) -> i32
{
    let mut sum=0;


    let mut len = calculatelength(num);
    let mut rem;
    while num>0 {
        rem =  num %10;
        sum  =  sum + rem.pow(len as u32);
        num = num/10;
        len = len-1;

    }
return sum;
}


// Enum in rust

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main()
{
   let color = Color::RGB(122, 17, 40);

    println!("What color is ?");

    match  color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is blue!"),
        Color::Green =>println!("The color is green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
    }
}



#[allow(dead_code)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction: Direction = Direction::Up;
    match player_direction
    {
        Direction::Up => println!("We are all heading up"),
        Direction::Down => println!("We are all going down"),
        Direction::Left =>println!("We are all going left"),
        Direction::Right =>println!("We are all going right"),
    }

    let player_direction: Direction = Direction::Down;
    match player_direction
    {
        Direction::Up => println!("We are all heading up"),
        Direction::Down => println!("We are all going down"),
        Direction::Left =>println!("We are all going left"),
        Direction::Right =>println!("We are all going right"),
    }

    let player_direction:Direction= Direction::Left;
    match player_direction
    {
        Direction::Up=>println!("We are all heading up"),
        Direction::Down => println!("We are all going down"),
        Direction::Left =>println!("We are all going left"),
        Direction::Right =>println!("We are all going right"),

    }

    let player_direction:Direction= Direction::Right;
    match  player_direction
    {
        Direction::Up=>println!("We are all heading up"),
        Direction::Down => println!("We are all going down"),
        Direction::Left =>println!("We are all going left"),
        Direction::Right =>println!("We are all going right"),
    }

}



// Structure in rust

struct Color{
    red : i32,
    green : u8,
    blue : u8

}

fn main()
{
    let mut bg = Color{red:23, green:67, blue:86};

    bg.green = 65;
    bg.red = 554;
    bg.blue = 43;
    println!("{},{},{}", bg.red, bg.green, bg.blue);

}



 */

//mod Program;
//  Struct program 2:

// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a >(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

}













