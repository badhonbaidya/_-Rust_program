//(1)//Rust Variables////********************************************************************************************

fn main() {
  let mut x = 5;
  x = 10;
  println!("After: {}\n befoure: {}",x, x);
}

//(2)//Rust Data Types//********************************************************************************************

fn main() {
  let my_num = 5;         // integer
  let my_double = 5.99;   // float
  let my_letter = 'D';    // character
  let my_bool = true;     // boolean
  let my_text = "Hello";  // string

  println!("my_num = {}\n my_duble = {}\n my_letter = {}\n my_bool = {}\n my_text = {}", my_num,my_double,my_letter,my_bool,my_text);
}

//(3)//Rust Constants//*********************************************************************************************

fn main() {
  const BIRTHYEAR: i32 = 1980;
  
  println!(" {} ",BIRTHYEAR);
}
********************************
  fn main() {
  const BIRTHYEAR:i64 = 19819876548765467;
  println!("What is your birth year? {}", BIRTHYEAR);
}
******************************************************
  fn main() {
  const BIRTHYEAR:u32 = 1981;
  println!("What is your birth year? {}", BIRTHYEAR);
}

//(4)// ************************************************************************************************************

fn main() {
  let add = 5+3;
  let sub = 5-3;
  let mul = 5*3;
  let div = 5/3;
  let rem = 5%3;
  
  println!("add : {}",add);
  println!("sub : {}",sub);
  println!("mul : {}",mul);
  println!("div : {}",div);
  println!("rem : {}",rem);
  
}

// (5)//Rust operators// *********************************************************************************************

fn main() {
  let mut x = 10;
  println!( "badhon = {}",x );
  
  x +=5;
  println!("after {}",x);
  
  x -=5;
  println!("after {}",x);
  
  x *=5;
  println!("after {}",x);
  
  x /=5;
  println!("after {}",x);
  
  x %=5;
  println!("after {}",x);
 
}

//comparison operators//********************
fn main() {
  let a = 10;
  let b = 20;
  
  println!("10 == 20: {}", a==b);
  println!("10 != 20: {}", a!=b);
  println!("10 <! 20: {}", a<!b);
  println!("10 < 20: {}", a<b);
  println!("10 >= 20: {}", a>=b);
 
}
//Logical Operators//****************************
fn main() {
  let logged_in = true;
  let is_admin = false;

  println!("Is regular user: {}", logged_in && ! is_admin);
  println!("Has any access: {}", logged_in || is_admin);
  println!("Not logged in: {}", !logged_in );
}

//{6}//Rust Booleans************************************************************************************************

 fn main() {
  let age = 20;
  let badhon = age >= 18;

  println!(" badhon age : {}", badhon);
}



////{7}//Rust If .. Else Conditions **********************************************************************************

//If///

fn main() {

if 10 > 5 {

println!("10 is the great 7");


}

}

//if//*************

fn main() {

let x = 12 ;
let y = 10 ;
if x > y {

println!(" x are then y ");
}
}

//if...else//*******************

fn main(){
 let age = 16;
 
 if age >= 18{
 println!("you can vote");
 
 }else{
 
 println!("badhon");
 
 }
 
}

// else if // ***********

fn main(){
 let score = 40;
 if score >= 90{
 println!("gread = a");
 
 }else if score >=80 {
 println!("gread = b");
 
 }else if score >=70 {
 println!("gread = c");
 
 }else if score >=60 {
 println!("gread = d");
 
 }else if score >= 50{
 println!("gread = e");
 
 }else{
 
 println!("gread = f");
 }
 }


 //Using if as an Expression//*****


 fn main(){
 
 let time = 20;
 let badhon = if time < 18{
 "good day"
 }else{
 "good even"
 };
 println!("{}", badhon )
 }


// Rust Match//******************************************************************************************************************

// When you have many choices, using match is easier than writing lots of if...else//
fn main() {
  let day = 5;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }
}

//You can match multiple values at once using the | operator (OR)://******

fn main() {
  let day = 2;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("sunday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}

//Just like if, match can also return a value://******

fn main() {
  let day = 8;

  let result = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

  println!("{}", result);
}

//Rust Loops//*********************************************************************************************************************
//loop is the simplest of Rust's three loop types.//*****

fn main() {
 let mut count = 1;
 loop{
 println!("badhon is a good boy");
 if count == 3 {
 break;
 
 }
 //count = count + 1;
  count += 1;
 }
 
 }

 //You can also return a value from a loop using break with a value.//****
fn main() {
  let mut count = 1;

  let result = loop {
    println!("Hello badhon");

    if count == 4 {
      break count; 
    }

    count += 1;
  };

  println!("The loop stopped at: {}", result);
}

//Rust While Loops//*********************************************************************************************************************
fn main() {
  let mut count = 2;

  while count <= 6 {
    println!("Count: {}", count);
    count += 1;
    //count = count+ 1;
  }
}
//You can stop a while loop when you want by using break://*******
fn main() {
  let mut num = 1;

  while num <= 10 {
    if num == 6 {
      num += 1;
     continue;
      
    }

    println!("Number: {}",num );
    num += 1;
    //num = num +1;
  }
}

//while loop://**************************************************************************************************************

fn main (){
 for i in 1..6 {
 println!("i is : {}", i);
 }
 }

 //If you want to include the last number, use ..= (two dots and an equals sign)://********

fn main (){
 for i in 1..= 7 {
 println!("i is : {}",i);
 }
 
 }
//ust like other loops, you can use break to stop the loop and continue to skip a value://*****
 fn main() {
  for i in 1..=10 {
    if i == 2 {
      continue; 
    }
    if i == 6 {
      break;  
    }  
    println!("i is: {}", i);
  }
}

//Rust Functions//***************************************************************************************************************
To call a function, write the name of the function followed by two parantheses ().//
fn main() {
  fn  say_hello() {
    println!("Hello from a function the badhon");
  }

   say_hello();
}

//You can send information into a function using parameters. Parameters are written inside the parentheses ().//***
fn main (){
 fn greet(name: &str) {
 
 println!("hello : {}",name);
 }
 greet ("badhon");
 }

// Use the  -> symbol in the function header to show what type of value will be returned.//******

fn main() {
  fn add(a: i32, b: i32) -> i32 {
    return a + b;
  }

  let sum = add(3, 4);
  println!("Sum is: {}", sum);
}

//In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon://****
fn main (){
 fn add (a : i32 , b : i32 ) -> i32 {
 a + b 
 }
 
 let sum = add (4,4);
 println!("sum is : {} ",sum);
 }


//Rust Scope//*******************************************************************************************************************
//In Rust, you can declare a new variable with the same name in the same scope using let. This is called shadowing://
fn main() {
  let mut x = 5;
  let y = 10;
  println!("x is: {}  {} ", x , y);
}

// The second x replaces the first one. The value 5 is no longer accessible after the second declaration.//
fn main (){
 let x = 20;
 
 {
 let x = 10;
 println!("inside block : {} ", x);
 
 }
 
 println!("outside block : {} ", x);
 }

 //Rust Strings//*******************************************************************************************************************
//You have already learned that you can use the &str type to create a string://
fn main(){
 let greeting : &str = "hello";
 println!(" badhon is : {}",greeting);
 
 }

 //Create a String
//You can create a String from a string literal using the to_string() method or the String::from() function://*****

fn main (){
 let text1 = "hello badhon baidya ". to_string();
 println!("{}",text1);
 
 }

 //****//
fn main() {
  let text2 = String::from("Hello badhon");
  println!("{}", text2);
}

/*Change a String
Use push_str() to add text to a string:*/********
fn main() {
  let mut greeting = String::from("Hello");
  greeting.push_str(" badhon");
  println!("{}", greeting);
}

//Use push() to add one character://*****
fn main() {
  let mut word = String::from("badhon");
  word.push('!');
  println!("{}", word);
}

//You can combine strings using the format! macro://****
fn main() {
  let s1 = String::from("Hello");
  let s2 = String::from("World!");
  let s3 = String::from("What a beautiful day");
  let s4 = String::from("in a badhon !");
  let result = format!("{} {} {} {}", s1, s2, s3,s4);
  println!("{}", result);
}

//You can also use the + operator to combine strings, but it can get messy with many values.//

fn main() {
  let s1 = String::from("Hello");
  let s2 = String::from("World!");
  let s3 = String::from("What a beautiful day");
   
  let result = s1 + " " + &s2 + " " + &s3 ;
  println!("{}", result);
}

//You can use the .len() method to get the length of a string://
fn main() {
  let name = String::from("badhon baidya");
  println!("badhon: {}", name.len());
}



