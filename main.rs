
//Rust Scope//*********************************************************************************************************************
//In Rust, you can declare a new variable with the same name in the same scope using let. This is called shadowing://
fn main() {
  let x = 5;
  let x = 10;
  println!("x is: {}", x);
}

fn main() {
  let x = 5;

  {
    let x = 10;
    println!("Inside block: {}", x);
  }

  println!("Outside block: {}", x);
}

//Rust Strings//*******************************************************************************************************************

fn main() {
  let greeting: &str = "Hello";
  println!("{}", greeting);
}
//You can create a String from a string literal using the to_string() method or the String::from() function://***
fn main() {
  let text1 = "Hello World".to_string();
  println!("{}", text1);
}

/*Strings are mutable, so you can change them if they are declared with mut.

Use push_str() to add text to a string:*/*******************
fn main() {
  let mut greeting = String::from("Hello");
  greeting.push_str(" badhon");
  println!("{}", greeting);
}
//****
fn main() {
  let mut word = String::from("Hi");
  word.push('!');
  println!("{}", word);
}

//You can combine strings using the format! macro://****
fn main() {
  let s1 = String::from("Hello");
  let s2 = String::from("World!");
  let s3 = String::from("What a beautiful day!");
  let result = format!("{} {} {}", s1, s2, s3);
  println!("{}", result);
}

//You can also use the + operator to combine strings, but it can get messy with many values.//********

fn main() {
  let s1 = String::from("Hello");
  let s2 = String::from("World!");
  let s3 = String::from("What a beautiful day!");
  let result = s1 + " " + &s2 + " " + &s3;
  println!("{}", result);
}

//You can use the .len() method to get the length of a string://****
fn main() {
  let name = String::from("badhon");
  println!("Length: {}", name.len());
}

//Rust Ownership//************************************************************************************************************
//In this example, a owns the string. Then we move it to b://
fn main() {
  let a = String::from("Hello");
  let b = a;

  // println!("{}", a);
  println!("{}", b);  
}
//When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.//***
fn main() {
  let a = 5;
  let b = a;
  println!("a = {}", a);
  println!("b = {}", b);
}

//String, if you really want to keep the original value and also assign it to another variable, you can use the .clone()//**
fn main() {
  let a = String::from("Hello");
  let b = a.clone(); // Now both have the same value

  println!("a = {}", a);
  println!("b = {}", b);
}

//Rust Borrowing and References//*************************************************************************************************
fn main() {
  let a = String ::from("Hello badhon");
  let b = &a;

  println!("a = {}", a);
  println!("b = {}", b);
}

/*Mutable References
If you want to change a value through a reference, you need to make the reference mut:*/

fn main() {
  let a = String::from("Hello badhon");
  let b = &a;

  println!("a = {}", a);
  println!("b = {}", b);
}


//Rust Data Structures//*****************************************************************************************************
fn main (){
let fruits = ["apple", "banana","orange"];
println!("last fruits : {}", fruits [1])

}

//Vectors//********

fn main (){
let mut fruits = vec ! ["apple","banana"];
fruits.push("orange");
println!("last fruits : {}",fruits[2]);
}

//Tuples//***************

fn main (){
let person = ("badhon","24","true");
println!("name : {} ",person.0);
println!("age : {} ",person.1);
println!("is active : {} ",person.2);

}

//HashMaps//***************
use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();
  capitalCities.insert("France", "Paris");
  capitalCities.insert("bangladesh", "dhaka");

  println!("Capital of dhaka is {}", capitalCities["bangladesh"]);
}

//Rust Arrays//******************************************************************************************************************
fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("The first number is: {}", numbers[1]);
}
//Change Array Values//******
fn main() {
  let mut numbers = [1, 2, 3, 4, 5];
  numbers[0] = 10;
  println!("The new first number is: {}", numbers[0]);
}

//Array Length//*****
fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("This array has {} elements.", numbers.len());
}

//Loop Through an Array//*******
fn main() {
  let fruits = ["apple", "banana", "orange"];
  for fruit in fruits {
    println!("I like {}.", fruit);
  }
}

/*Print the Entire Array
Note: When printing the whole array, you must use {:?} inside println!:*/

fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);
}

////*****************************
fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("{}", numbers[0]);
}
//Vectors - Dynamic Size Example//**************
fn main() {
  let mut cars = vec!["Volvo", "BMW", "Ford"]; 
  cars.push("Mazda");

  println!("{:?}", cars); 
}

//Rust Arrays//******************************************************************************************************************
//This statement accesses the value of the first element [0] in numbers://*******
fn main (){
let number = [1,2,3,4,5,6,];
println!("this is my first number : {}", number [4]);

}

//Change Array Values//**************
//Remember to make the array mutable (using the mut keyword)://****
fn main() {
  let mut numbers = [1, 2, 3, 4, 5];
  numbers[0] = 10;
  println!("The new first number is: {}", numbers[0]);
}

//Array Length
//You can get the number of elements in an array using the .len() method://****
fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("This array has {} elements.", numbers.len());
}

//Loop Through an Array
//You can loop through the array elements with the for loop.//****

fn main() {
  let mut fruits = ["apple", "banana", "orange"];
  for fruit in fruits {
    println!("I like {}.", fruit);
  }
}

//Print the Entire Array
//Note: When printing the whole array, you must use {:?} inside println!://

fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);
}

//If you are just printing one element from the array, you can use {}.//

fn main() {
  let numbers = [1, 2, 3, 4, 5];
  println!("{}", numbers[0]);
}

//Example//***************

fn main() {
 
  let mut cars = vec!["Volvo", "BMW", "Ford"];
 
  cars.push("Mazda");

  println!("{:?}", cars); 
}

//Rust Vectors//******************************************************************************************************
fn main() {
  let fruits = vec!["apple", "banana", "orange" ];
  println!("First fruit: {}", fruits[0]);
}

//Change Vector Values//*********
fn main() {
  let mut fruits = vec!["apple", "banana", "orange"];
  fruits[0] = "grape";
  println!("New first fruit: {}", fruits[0]);
}

//You can add a new element to the end of a vector using the push() method://*****

fn main() {
  let mut fruits = vec!["apple", "banana"];
  fruits.push("cherry");
  println!("{:?}", fruits);
}
