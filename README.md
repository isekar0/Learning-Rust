# Learning-Rust
Repo where I can upload code and notes as I learn Rust


## Rust Language Book

### Chapter 1

##### Functions and macros
-> "functions" ending with "!" are actually macros? (See chapter 20 for more on macros)

##### Building
-> By default,  "$ cargo build ..." builds with debug flags, "$ cargo build --release ..." builds a release-ready target with optimizations
-> --release also disables default panics like checking for integer overflow

##### Directories, TOML, and "[[workspace]]"
-> While in Learning-Rust/, I could not use commands like "$ cargo run guessing-game" to build nested programs. First I had to run "$ cargo init" and setup a "[[workspace]]" section which contained the other crates, i.e. guessing-game and hello-rust

### Chapter 2

##### Printing
-> The formatting seems to be a mix of Python's f-strings and C's printf
e.g. println!("{variable_name_initialized_above} = {variable_here}", function());

Status update: Just used a dependency for the first time, nice.

Cargo.lock is where dependencies get frozen, only updated when you explicitly change the version in Cargo.toml, or use "$ cargo update"

##### Ranges
-> A range expression can take the form of start..=stop which is inclusive on the lower and upper bounds
-> There is also start..stop which is exclusive

##### Documentation
-> "$ cargo doc --open" generates documentation for your project and opens it in html format. 
-> "$rustc --explain <ERROR_CODE>" can generate documentation on the error and provide some examples

##### Typing
-> Rust has both a strong, static type system while also being able to infer types 

##### Rusty coding style
-> According to rust documentation, it is more rusty to write a function which only returns an expression, as in it doesn't do any other functionality, as a one-liner, omitting the return keyword and even semicolon! Wow, pretty crazy... but I wish to be rusty :3
    -> Okay, maybe to correct this, but Rust is an "expression" based language (?), so for example, let x = 5; is a statement, and statements don't return values and end with a semicolon, while as expressions return values (and don't always end in semicolons?), that's why if it's the last line in a function with a return type, you don't include the semicolon, if you did, it wouldn't return anything, that's what the error message means with '()', that's the unit type which is a statement.

##### Loops
-> literal "loop" keyword. I guess they just made while (true) {...} have less boilerplate? 

### Chapter 3

##### Chars
-> Unlike C, char is 4 bytes and represents unicode scalar values

##### Tuples
-> Unlike Python, tuples have the nature of being immutable in terms of size, while the mutability of their elements is dependant on Rust's mutability 
-> Elements of a tuple are accessed like that of a struct rather than a list
e.g. 
tup_variable.0 

##### Arrays
-> Like C, they are fixed in size and all elements are of the same type
-> Always on the stack? In comparison, vectors can grow and shrink but are always on the heap?
-> Can be defined without type or size, if all elements are explicitly initialized, or can be declared with a type and size, or can be declared by the sole unique element and the size of the array
e.g.
let a = [1, 2, 3];
let a : [u32; 3] = [1, 2, 3];
let a = [0, 3];

##### Functions
-> Unlike C, you cannot have prototype functions, but you can declare functions anywhere in the user scope

##### Control flow
-> Like C, there is something similar to the ternary operator
e.g.
let _boolean = true;
let x = if _boolean {1} else {0};
--> Values resulting from each branch must be of the same type

##### Boolean
-> There are no truthy or falsy values in Rust, i.e. any non-zero number is not equivalent to true. Conditional statements like if require a boolean value, which can only be true or false.

##### Loops
-> In addition to for and while loops there is also the loop keyword which is equivalent to while (true)
-> variables can be declared as the result of a loop
e.g.
let mut i = 0;
let x = loop {i+=1; if i == 10 {break i * 2;}};
which will result in x having a value of 20
--> return can be used in place of break, and thusly the semicolon is optional for both cases

**Loop Labels**
--> You can label loops with 'name: loop {...} (notice the single apostrophe on the left hand-side of the loop name). This can then be used in the loop to, for example break the outer body of the loop from within another loop.
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

##### Ownership
-> Unlike low-level languages, memory cannot be thought of as an array of bytes, since Rust does not allow that level of manipulation
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

To avoid moving data, clone it, now instead of having transferring ownership to a new pointer, you create a new pointer that points to it's own clone of the original data (this is a deep copy, in contrast to shallow copies which is what happens with moving data)
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


##### Dereferencing
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
fn main()  {
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
}

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

Data races occur when these 3 conditions are satisfied
    1) Two or more pointers access the same data at the same time. 
    2) At least one of the pointers is being used to write to the data. 
    3) There’s no mechanism being used to synchronize access to the data. 

So if a pointer points to a mutable reference of some data, then usually it has to first be dropped before another pointer to a mutable reference of that data can be made.

fn main() {
    let mut s = String::from("hello"); 
    let r1 = &s; // no problem 
    let r2 = &s; // no problem 
    let r3 = &mut s; // BIG PROBLEM 
    println!("{r1}, {r2}, and {r3}"); 
}

-> This code also does not compile because you cannot have a mutable reference to a piece of data when an immutable reference to it exists, after all, if you mutate the mutable reference, then that's sort of bypassing the immutable reference's immutability

The Rules of References
    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.

#### Slices 
Referencing a contiguous sequence of elements.

Created with [starting_index..ending_index]

e.g. 
fn main() {
    let s = String::from("hello world"); 
    
    let hello = &s[0..5]; // Which is equal to ... [..5] i.e. if starting_index = 0
    let world = &s[6..11]; // Which is equal to ... [6..] i.e. if ending_index = len
}

N O T E  
String slice range indices must occur at valid UTF-8 character boundaries. If you  attempt to create a string slice in the middle of a multibyte character, your program  will exit with an error.

### Chapter 5

#### Structs

The names of the thingies in a struct is "fields"

Either the entire instance of a struct is mutable or immutable, and all fields will follow suit.

Field init shorthand syntax
-> If you initialize an instance of a struct with a variable with the same name as the field it is being assigned to, you can omit most of the assignment and just write the name alone.

Struct update syntax
-> when creating a new instance which shares fields with another, first declare any unique fields as usual, then use "..other_instance_name"
-> Because this still uses "=", it moves the data from the previous instance to the new one, so the previous one cannot be used anymore if heap-allocated data was moved (if only i32, u32, (types that implement the copy trait), were moved then the previous one can still be used, but if it included data that needs to be dropped at the end of its scope, then ownership has to be transferred to the new instance during movement)

**Tuple structs** 
-> Merging the ability to give something a name from the struct type and having nameless fields from the tuple type
e.g.
struct Color(u8, u8, u8);
The fields only have a type, like a tuple, but the "object"(?) has a name which can be used to create instances of it

**Unit-like structs**
-> Structs without fields
-> Behave similarly to the unit type ()
-> Used when you want to implement a trait on some type but don't have any data that you want to store in the type itself
e.g.
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

##### Ownership of struct data
-> A struct requires either owned types like String, or a lifetime specifier for references like &str

##### Derived traits
an example of which is 
#[derive(debug)] // outer attribute which opts in for Rust's debug functionality for this struct type, Rust derives a debug trait for this struct type
struct ...
--> Allows you to use println!("{:?}", ...) to print out the struct, 
---> Although it's better to use the dbg! macro since it returns ownership of the value (and also returns to stderr instead of stdout)
    e.g. 
    ...
    let rectangle = Rectangle {
        width: dbg! (30 * scale), // evaluates the expression, displays the result in stderr, and returns ownership of the result
        height : 50,
    };
    
##### Associated Functions
-> Defined within the context of a struct, enum, or a trait object
-> implementations of a struct
-> May or may not have the self parameter, if they do they are a method. A common example of an associated function that doesn't have self as an input parameter is a constructor
--> Which then uses the double colon :: syntax 

###### Methods
-> Like Python, their first parameter is self
e.g.
struct Rectangle { 
    width: u32,
    height: u32,
}

impl Rectangle { // implementation for the Rectangle struct, can hold multiple definitions inside
    fn calc_area(&self) -> u32 { // &self is shorthand for self : &Self where the Self type is an alias for the type the implementation is for 
        self.width * self.height
    }
}

...
--> Methods can also take ownership of self, borrow immutably, or borrow mutably

**Automatic (de)referencing**
-> Unlike C/C++ where you need to use object->something() (i.e. (*object).something()), Rust automatically adds in the &, &mut, or * to match the signature of the method 

### Chapter 6
