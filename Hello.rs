// This is single line comment
/*
 * This is a multi line comment
 */
fn main() {
  println!("Rust says Hello to DAO TIEN TU !!"); // prints just a newline
  println!("format {} arguments", "some"); // prints format some arguments
  // Data Types
  // Declare a variable
  let company_string = "TutorialsPoint"; // string type
  let rating_float = 4.5; // float type
  let is_growing_boolean = true; // boolean type
  let icon_char = '♥'; // unicode character type
  println!("company name is: {}", company_string);
  println!("company rating on 5 is: {}", rating_float);
  println!("company is growing: {}", is_growing_boolean);
  println!("company icon is: {}", icon_char);
  // 4 primary scalar types: integer, floating-point, booleans, characters
  // Integer
  let result = 10; // i32 by default: 32 bit
  let age:u32 = 20; // unsigned 32 bit
  let sum:i32 = 5-15;
  let mark:isize = 10; // size Arch
  let count:usize = 30;
  println!("result value is {}", result);
  println!("sum is {} and age is {}", sum, age);
  println!("mark is {} and count is {}", mark, count);
  // Integer overflow
  //let age:u8 = 255; // unsigned 8 bit
  // Float
  let result = 10.00; // f64 by default: double precision float
  let interest:f32 = 8.35; // f32: single precision float
  let cost:f64 = 15000.600; // double precision
  println!("Result value is {}", result);
  println!("Interest is {}", interest);
  println!("Cost is {}", cost);
  // Number separator
  let float_with_separator = 11_000.555_001;
  println!("float value {}", float_with_separator);
  let int_with_separator = 50_000;
  println!("int value {}", int_with_separator);
  // Boolean
  let is_fun:bool = true;
  println!("Is Rust Programming Fun ? {}", is_fun);
  // Character
  let special_character = '@'; // default
  let alphabet: char = 'A';
  let emoji: char = '😁';
  println!("Special character is {}", special_character);
  println!("Alphabet is {}", alphabet);
  println!("Emoji is {}", emoji);
  // Variables are immutable by default
  // Prefix the variable name with mut keyword to make it mutable
  let mut fees:i32 = 25_000;
  println!("Fees is {}", fees);
  fees = 35_000;
  println!("fees changed is {}", fees);
  // Constant
  const USER_LIMIT:i32 = 100; // Declare a integer constant
  const PI:f32 = 3.14; // Declare a float constant
  println!("User limit is {}", USER_LIMIT);
  println!("pi limit is {}", PI);
  // Shadowing of variables, constants cannot be shadowed
  let salary = 100.00;
  let salary = 1.50;
  // reads first salary
  println!("The value of salary is: {}", salary);
  let uname = "Mohtashim";
  let uname = uname.len();
  println!("Name changed to integer: {}", uname);
  // String Literal(&str)
  let company:&str = "TutorialsPoint";
  let location:&str = "Hyderabad";
  println!("Company is: {} location: {}", company, location);
  // String literals are static by default
  let company: &'static str = "TutorialsPoint";
  let location: &'static str = "Hyderabad";
  println!("Company is: {} location: {}", company, location);
  // String Object
  // Create a string object (an empty string)
  let empty_string = String::new();
  println!("Length is {}", empty_string.len());
  // Create a string with some default value passed as parameter to the from() method
  let content_string = String::from("TutorialsPoint");
  println!("length is {}", content_string.len());
  /*
   * Illustration:new()
   * An empty string object is created using the new() method and its value is set to hello
   */
  let mut z = String::new();
  z.push_str("hello");
  println!("{}", z);
  /*
   * Illustration:to_string()
   * To access all methods of String object, convert a string literal to object type using the to_string() function
   */ 
  let name1 = "Hello TutorialsPoint, Hello!".to_string();
  println!("{}", name1);
  /*
   * Illustration:replace()
   */
  let name1 = "Hello TutorialsPoint, Hello!" . to_string(); // String object
  let name2 = name1.replace("Hello", "Howdy"); // Find and replace
  println!("{}", name2);
  /*
   * Illustration:as_str()
   */
  println!("Displaying string literal {}", String::from("example_string").as_str());
  /*
   * Illustration:push()
   * function appends the given char to the end of this string
   */
  let mut company = "Tutorial" . to_string();
  company.push('s');
  println!("{}", company);
  /*
   * Illustration:push_str()
   * function appends the given string slice onto the end of this string
   */
  let mut company = "Tutorials" . to_string();
  company.push_str(" Point");
  println!("{}", company);
  /*
   * Illustration:len()
   */
  let fullname = " Tutorials Point";
  println!("length is {}",fullname.len());
  /*
   * Illustration:trim() 
   */
  let mut fullname = " Tutorials Point \r\n";
  println!("Before trim ");
  println!("Length is {}", fullname.len());
  println!();
  println!("After trim ");
  println!("Length is {}", fullname.trim().len());
  /*
   * Illustration:split_whitespace()
   * function splits the input string into different strings
   */
  let msg = "Tutorials Point has good tutorials" . to_string();
  let mut i = 1;
  for token in msg.split_whitespace() {
    println!("token {} {}", i, token);
    i+=1;
  }
  /*
   * Illustration: split() string
   * method returns an iterator over substrings of a string slice, separated by characters matched by a pattern.
   * collect method can be used to store the result returned by split() as a vector 
   */
  let fullname = "Kannan,Sudhakaran,Tutorialspoint";
  for token in fullname.split(",") {
    println!("Token is {}", token);
  }
  // store in a vector
  println!("\n");
  let tokens:Vec<&str>=fullname.split(",").collect();
  println!("First name is {}", tokens[0]);
  println!("Last name is {}", tokens[1]);
  println!("Company is {}", tokens[2]);
  /*
   * Illustration: chars() 
   */ 
  let n1 = "Tutorials" . to_string();
  for n in n1.chars() {
    println!("{}", n);
  }
  /*
   * Concatenation of Strings with + operator 
   */
  let n1 = "Tutorials" . to_string();
  let n2 = "Point" . to_string();
  let n3 = n1 + &n2; // n2 reference is passed
  println!("{}", n3);
  /*
   * Type Casting 
   */
  let number = 2020;
  let number_as_string = number.to_string();
  // convert number to string
  println!("{}", number_as_string);
  println!("{}", number_as_string=="2020");
  /*
   * Format! Marco 
   * another way to add to String objects together
   */
  let n1 = "Tutorials" . to_string();
  let n2 = "Point" . to_string();
  let n3 = format!("{} {}", n1, n2);
  println!("{}", n3);
  let num:i32 = 5;
  if num > 0 {
    println!("Number is positive");
  }
  let num = 12;
  if num % 2 == 0 {
    println!("Even"); // Số chẵn
  } else {
    println!("Odd"); // Số lẻ
  }
  // Nested if
  let mut num = 2;
  if num > 0 {
    println!("{} is positive", num);
  } else if num < 0 {
    println!("{} is negative", num);
  } else {
    println!("{} is neither positive nor negative", num);
  }
  // Match statement
  // The match statement checks if a current value is matching from a list of values
  let state_code = "MH";
  let state = match state_code {
    "MH" => {println!("Found match for MH"); "Maharashtra"},
    "KL" => "Kerala",
    "KA" => "Karnadaka",
    "GA" => "Goa",
    _ => "Unknown" // default
  };
  println!("State name is {}", state);
  // Definite Loop: Vòng lặp xác định
  for x in 1..11 { // 11 is not inclusive
    if x == 5 {
      continue; // Skip value 5
    }
    println!("x is {}", x)
  }
  // Indefinite Loop: Vòng lặp vô hạn
  let mut x = 0;
  while x < 10 {
    x+=1;
    println!("Inside loop x value is {}", x);
  }
  println!("Outside loop x value is {}", x);
  // Break
  let mut x = 0;
  loop {
    x+=1;
    if x==15 {
      break;
    }
    println!("x={}", x);
  }
  // Continue
  let mut count = 0;
  for num in 0..21 {
    if num % 2 == 0 {
      continue;
    }
    count+=1;
  }
  println!("The count of odd values between 0 and 20 is: {} ", count);
  // Calling a function
  fn_hello();
  println!("pi value is {}", get_pi());
  let no:i32 = 5;
  mutable_no_to_zero(no);
  println!("The value of no is: {}", no);
  let mut no:i32 = 5;
  pass_by_reference(&mut no);
  println!("The value of no is: {}", no);
  let name:String = String::from("TutorialsPoint");
  display(name);
  // Tuple
  /*
  Tuple is a compound data type. A scalar type can store only type of data
  */
  let tuple:(i32, f64, u8) = (-325, 4.9, 22);
  println!("Integer is: {:?}", tuple.0);
  println!("Float is: {:?}", tuple.1);
  println!("Unsigned integer is: {:?}", tuple.2);
  // The following example passes a tuple as parameter to a function. Tuples are passed by value to functions
  let b:(i32, bool, f64) = (110, true, 10.9);
  print(b);
}
// Defining a function
fn fn_hello() {
  println!("Hello from function fn_hello");
}
fn get_pi()->f64 {
  22.0/7.0
}
// Function with parameters
fn mutable_no_to_zero(mut param_no: i32) {
  param_no = param_no * 0;
  println!("Param_no value is: {}", param_no);
}
// Pass by reference
fn pass_by_reference(param_no:&mut i32) {
  /*
  The * operator is used to access value stored in the memory location that the variable param_no points to. This is also known as dereferencing
  */
  *param_no = 0; // de reference
}
fn display(param_name:String) {
  println!("param_name value is: {}", param_name);
}
fn print(x:(i32, bool, f64)) {
  println!("Inside print method");
  println!("{:?}", x);
  // Destructing
  // Assignment is a feature of rust wherein we unpack the values of a tuple. This is achieved by assigning a tuple to distinct variables
  println!("Inside print method");
  let (age, is_male, cgpa) = x; // assigns a tuple to distinct variables
  println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
  // Declaring and Initializing arrays
  /*
  Syntax 1:
  let variable_name = [value1, value2, value3];
  Syntax 2:
  let variable_name:[dataType;size] = [value1, value2, value3];
  Syntax 3:
  let variable_name:[dataType;size] = [default_value_for_elements, size];
  */
  let arr:[i32;4] = [10, 20, 30, 40];
  println!("array is: {:?}", arr);
  println!("array size is: {}", arr.len());
  // Array without data type
  let mut arr = [10, 20, 30, 40];
  println!("array is: {:?}", arr);
  println!("array size is: {}", arr.len());
  // Default values
  let arr:[i32;4] = [-1;4];
  println!("array is {:?}", arr);
  println!("array size is: {}", arr.len());
  // Array with for loop
  for index in 0..4 {
    println!("index is: {} & value is: {}", index, arr[index]);
  }
  // using the iter() function
  /*
  The iter() function fetches values of all elements in an array
  */
  for val in arr.iter() {
    println!("value is: {}", val);
  }
  // mutable array
  /*
  The mut keyword can be used to declare a mutable array. The following example declares a mutable array and modifies value of the second
  array element
  */
  let mut arr:[i32;4] = [10, 20, 30, 40];
  arr[1] = 0;
  println!("{:?}", arr);
  update(&mut arr);
  // arr = [0, 0, 0, 0]
  // Array declaration and constants
  const N:usize = 20;
  // pointer sized
  let arr = [0; N];
  print!("{}", arr[10]);
}
// pass by value
fn update(arr:&mut[i32;4]) {
  for i in 0..4 {
    arr[i] = 0;
  }
  println!("Inside update {:?}", arr);
  // Ownership
  /*
  The memory for a program can be allocated in the following
  - Stack
  - Heap
  A stack follows a last first out order
  */
  /*
  The heap is an area in the momory which is less organized when compared to stack
  */
  // Assigning value of one variable to another variable
  let v = vec![1, 2, 3];
  // vector v owns the object in heap

  // only a single variable owns the heap menmory at any given time
  let v2 = v;
  // here two variables owns heap value, two pointers to the same content is not allowed in rust
  // Rust is very smart in terms of memory access, so it detects a race condition as two variables point to same heap
  println!("{:?}", v2);
  // Ownership and primitive types
  let u1 = 10;
  let u2 = u1; // u1 value copied (not moved) to u2
  println!("u1 = {}", u1);
  // Borrowing
  let v = vec![10, 20, 30];
  print_vector(&v); // passing reference
  println!("Printing the value from main() v[0]={}", v[0]);
  // Mutable References (Tham chiếu có thể thay đổi)
  // Mutating an integer reference
  let mut i = 3;
  // Passes a mutable reference of i to the add_one()
  add_one(&mut i);
  println!("{}", i);
  // Mutating a string reference
  let mut name:String = String::from("TutorialsPoint");
  display1(&mut name);
  // Pass a mutable reference of name
  println!("The value of name after modification is: {}", name);
}
fn print_vector(x:&Vec<i32>) {
  println!("Inside print_vector function {:?}", x);
}
fn add_one(e:&mut i32) {
  *e+=1;
}
fn display1(param_name:&mut String) {
  println!("param_name value is: {}", param_name);
  param_name.push_str(" Rocks");
  // Modify the actual string, name
  let n1 = "Tutorials".to_string();
  println!("Length of string is {}", n1.len());
  let c1 = &n1[4..9];
  // fetches characters at 4, 5, 6, 7 and 8 indexes
  println!("{}", c1);
  // Slicing an integer array
  let mut data = [10, 20, 30, 40, 50];
  use_slice(&mut data[1..4]);
  // this is effectively borrowing elements for a while
  println!("{:?}", data);
}
fn use_slice(slice: &mut [i32]) {
  // is taking a slice or borrowing a part of an array of i32s
  println!("length of slice is {:?}", slice.len());
  println!("{:?}", slice);
  slice[0] = 1010; // replaces 20 with 1010
  // Declaring a structure
  /*
  struct Name_of_structure {
    field1: data_type,
    field2: data_type,
    field3: data_type
  }
  */
  let mut emp1 = Employee{
    company: String::from("TutorialsPoint"),
    name: String::from("Mohtashim"),
    age: 50
  };
  // Modifying a struct instance
  emp1.age = 40;
  println!("Name is {} company is {} age is {}", emp1.name, emp1.company, emp1.age);
  // Passing a struct to a function
  // Initialize a structure
  let mut emp2 = Employee {
    company:String::from("TutorialsPoint"),
    name: String::from("Kannan"),
    age: 32
  };
  // pass emp1 and emp2 to display()
  //display2(emp1);
  //display2(emp2);
  // Returning struct from a function
  let elder = who_is_elder(emp1, emp2);
  println!("elder is: ");
  // prints details of the elder employee
  display2(elder);
  // Method in structure
  /*
  Syntax:
  struct My_struct {}
  impl My_struct {
    //set the method's context
    fn method_name() {
      // define a method
    }
  }
  */
  // instanatiate the structure
  let small = Rectangle{
    width: 10,
    height: 20
  };
  // print the rectangle's area
  println!("width is {} height is {} area of Rectangle is {}", small.width, small.height, small.area());
  // Static method in structure
  // can be used as utility methods
  /*
  Syntax - Declaring a static method
  impl Structure_Name {
    // static method that creates objects of the Point structure
    fn method_name(param1: datatype, param2: datatype)->return_type{
      // logic goes here
    }
  }
  Syntax - Invoking a static method
  structure_name::method_name(v1, v2)
  */
  // invoke the static method
  let p1 = Point::getInstance(10, 20);
  p1.display3();
  // Enums
  /*
  Syntax:
  enum enum_name {
    variant1,
    variant2,
    variant3
  }
  */
  print_size(CarType::SUV);
  print_size(CarType::Hatch);
  print_size(CarType::Sedan);
  // Modules
  /*
  A logical group of code is called a module
  Multiple modules are compiled into a unit called crate.
  */
  play("Herold and Kunar" . to_string());
  // short path syntax
  play1("Airplane!" . to_string());
}
/*
Syntax:
// public module
pub mod a_public_module {
  pub fn a_public_function() {
    // public function
  }
  fn a_private_function() {
    // private function
  }
}
// private module
mod a_private_module {
  fn a_private_function() {
    // private function
  }
}
*/
// use keyword
/*
The use keyword helps to import a public module
Syntax:
use public_module_name::function_name;
*/
// Nested Modules
pub mod movies1 {
  pub mod english {
    pub mod comedy {
      pub fn play1(name:String) {
        println!("Playing comedy movie {}", name);
      }
    }
  }
}
// importing a public module
use movies1::english::comedy::play1;
pub mod movies {
  pub fn play(name:String) {
    println!("Playing movie {}", name);
  }
}
use movies::play;
enum CarType {
  Hatch,Sedan,SUV
}
fn print_size(car: CarType) {
  match car {
    CarType::Hatch => {
      println!("Small sized car");
    },
    CarType::Sedan => {
      println!("Medium sized car");
    },
    CarType::SUV => {
      println!("Large sized Sports Utility car");
    },
  }
}
struct Point {
  x: i32,
  y: i32,
}
impl Point {
  // static method that creates objects of the Point structure
  fn getInstance(x:i32,y:i32)->Point{
    Point{x:x,y:y}
  }
  // display values of the structure's field
  fn display3(&self) {
    println!("x={} y={}", self.x, self.y);
  }
}
// define dimensions of a rectangle
struct Rectangle {
  width: u32, height: u32
}
// Logic to calculate area of a rectangle
impl Rectangle {
  fn area(&self)->u32 {
    // use the .operator to fetch the value of a field via the self keyword
    self.width * self.height
  }
}
struct Employee {
  name: String,
  company: String,
  age: u32
}
// accepts instances of employee structure and compares their age
fn who_is_elder(emp1:Employee,emp2:Employee)->Employee{
  if (emp1.age > emp2.age) {
    return emp1;
  } else {
    return emp2;
  }
}
// fetch values of specific structure fields using the operator and print it to the console
fn display2(emp:Employee) {
  println!("Name is {} company is {} age is {}", emp.name, emp.company, emp.age);
}
