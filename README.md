# Learning-Rust
Repo where I can upload code and notes as I learn Rust


## Rust Language Book

### Chapter 1

Funcs and macros
-> "functions" ending with "!" are actually macros? (See chapter 20 for more on macros)

Building
-> By default,  "$ cargo build ..." builds with debug flags, "$ cargo build --release ..." builds a release-ready target with optimizations
-> --release also disables default panics like checking for integer overflow

Directories, TOML, and "[[workspace]]"
-> While in Learning-Rust/, I could not use commands like "$ cargo run guessing-game" to build nested programs. First I had to run "$ cargo init" and setup a "[[workspace]]" section which contained the other crates, i.e. guessing-game and hello-rust

### Chapter 2

Printing
-> The formatting seems to be a mix of Python's f-strings and C's printf
e.g. println!("{variable_name_initialized_above} = {variable_here}", function());

Status update: Just used a dependancy for the first time, nice.

Cargo.lock is where dependencies get frozen, only updated when you explicitely change the version in Cargo.toml, or use "$ cargo update"

Ranges
-> A range expression can take the form of start..=stop which is inclusive on the lower and upper bounds
-> There is also start..stop which is exclusive

Documentation
-> "$ cargo doc --open" generates documentation for your project and opens it in html format. 
-> "$rustc --explain <ERROR_CODE>" can generate documentation on the error and provide some examples

Typing
-> Rust has both a strong, static type system while also being able to infer types 

Rusty coding style
-> According to rust documentation, it is more rusty to write a function which only returns an expression, as in it doesn't do any other functionality, as a oneliner, omitting the return keyword and even semicolon! Wow, pretty crazy... but I wish to be rusty :3

Loops
-> literal "loop" keyword. I guess they just made while (true) {...} have less boilerplate? 

### Chapter 3

Chars
-> Unlike C, char is 4 bytes and represents unicode scalar values

Tuples
-> Unlike Python, tuples have the nature of being immutable in terms of size, while the mutability of their elements is dependant on Rust's mutability 
-> Elements of a tuple are accessed like that of a struct rather than a list
e.g. 
tup_varialbe.0 

Arrays
-> Like C, they are fixed in size and all elements are of the same type
-> Always on the stack? In comparision, vectors can grow and shrink but are always on the heap?
-> Can be defined without type or size, if all elements are explicitly initialized, or can be declared with a type and size, or can be declared by the sole unique element and the size of the array
e.g.
let a = [1, 2, 3];
let a : [u32; 3] = [1, 2, 3];
let a = [0, 3];

Functions
-> Unlike C, you cannot have prototype functions, but you can declare functions anywhere in the user scope

Control flow
-> Like C, there is something similar to the ternary operator
e.g.
let _boolean = true;
let x = if _boolean {1} else {0};
--> Values resulting from each branch must be of the same type

Boolean
-> There are no truthy or falsy values in Rust, i.e. any non-zero number is not equivalent to true. Conditional statements like if require a boolean value, which can only be true or false.

Loops
-> In addition to for and while loops there is also the loop keyword which is equivalent to while (true)
-> variables can be declared as the result of a loop
e.g.
let mut i = 0;
let x = loop {i+=1; if i == 10 {break i * 2;}};
which will result in x having a value of 20
--> return can be used in place of break, and thusly the semicolon is optional for both cases
**Loop Labels**
--> You can label loops with 'name: loop {...} (notice the single apostraphe on the left hand-side of the loop name). This can then be used in the loop to, for example break the outer body of the loop from within another loop.
e.g. 
let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
-->             break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

For loops
-> Like Python, you can iterate through the elements of array with 
for x in arr {...}
also
for x in (1..5) {...}

### Chapter 4

Ownership
-> Unlike low-level languages, memory cannot be thought of as an array of bytes, since Rust does not allow that level of manipualtion
Box
-> A way of allocating memory on the heap
e.g.
let x = Box::new([0; 1_000_000]);
-> Manual memory management, e.g. allocating and freeing, so not allowed in Rust. Rather, Rust manages memory based on scope, automatically providing enough memory when a process occurs, and freeing it when it goes out of scope
-> Heap data is deallocated whenever the last stack frame pointing to that data is dropped
-> example of ownership
    let a = Box::new([0; 100]);
    let b = a;

--> First, a owns this heap data, afterwards, when b is assigned to a, b owns the heap data
---> The pointer to the heap data was **moved** to b, when b is dropped by the stack frame, the pointer to the heap data is lost. It cannot be called again.

Variables cannot be used after being moved
e.g.
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

To avoid moving data, clone it
e.g.
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

Or reference it
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

Where g1 and g2 are pointers to m1 and m2 which are pointers to "Hello" and "world", respectively. g1 and g2 never own m1 and m2, nor do they ever own the two strings, which is why, when g1 and g2 are dropped, m1 and m2 are still valid; because the strings didn't get deallocated along with g1 and g2.

Summary
Ownership is primarily a discipline of heap management:

    - All heap data must be owned by exactly one variable.
    - Rust deallocates heap data once its owner goes out of scope.
    - Ownership can be transferred by moves, which happen on assignments and    function calls.
    - Heap data can only be accessed through its current owner, not a previous owner.


Dereferencing
fn main() {
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let c: i32 = *r2;    // so only one dereference is needed to read it
}

Rust often dereferences implicitly
#fn main()  {
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
#}

Aliasing and mutations (**Borrow Checker**)
-> Both are allowed on their own, but not in combination.
e.g.
fn main() {

let mut v: Vec<i32> = vec![1, 2, 3]; // v has read, write permission and ownership.

let num: &i32 = &v[2]; // v loses write permission and ownership, num gains ownership and read, and *num can only read

println!("Third element is {}", *num); // num is dropped and loses write permissions and ownership, v gains them back

v.push(4); // v is dropped (since it has to expand, it is not stored elsewhere in memory (also since main is out of the stack frame)), loses all perms
}
---> This prevents data races and memory corruption