use std::mem;
struct Object {
    width: u32,
    height: u32,
}
impl Object {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
        }
    }
    fn show(&self) {
        println!("{}", self.area());
    }
}

struct Point {
    x: i32,
    y: i32,
}
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

fn main() {
    // println!("Hello, world!");
    // learn_data_types();
    // learn_objects();
    let obj = Object::new(32, 32);
    // learn_conditions_flow();
    learn_enums_and_option();
}

fn learn_enums_and_option() {
    todo!()
}

fn learn_data_types(){
    // let x = 5;
    // x = 5; Cannot assign one immutable variable twice

    let mut _y;
    _y = 10;  // No Error, It's mutable
    println!("{}", _y);
    // primitives
    /*
        Integers: i8, u8, i16, u16, i32, i64, u64, isize, usize
        Float: f32 - single precision, f64 - double precision,
        Boolean - true / false
        Char - 'a' - one char with single quotes
        tuples: collections of data.    t:(i32, float, char) helps in destructuring - (42, 2.5, 'w')
        arrays: collection of one data type; let a = [1,2,3,4,5,6]
        => Can fit tuple inside tuple
    */
    // Tuple
    let t = (1,2,"learn");
    let f = (2, t);
    // println!("{:#?}", f );

    // Array
    let a:[i32; 5] = [4,5,6,8,1];
    // println!("{:?} {}", a, mem::size_of_val(&a)); // int occupies 4 bytes of memory


    // Important / String
    let _s = "one ";   // slice of string
    let s1 = "string ".to_string();   // slice of string -> string
    let ss = String::from("two"); // string
    let comb = s1.clone() + &ss;
    // println!("{}",comb);

    let t = (); // tuple unit value
    // function which doesn't return anything, returns empty tuple


    /*
        ownership - each variable has a value, and variable itself is an owner
    */
    let s = String::from("String"); // owner of string -> s
    let y = &s; //borrow for a moment
    // println!("{}", s);
}

fn learn_objects(){
    let o = Object{
        width: 16,
        height: 20
    };
    let obj = Object::new(32, 32);
    // obj.show();
    // o.show();
}

fn learn_conditions_flow(){
    let n = 2;
    if n<5 {
        println!("less than 5")
    }
    else {
        println!("greater than 5");
    }
    // binding a variable
    let c = true;
    let num = if c {
        50
    } else {
        60
    };
    println!("{}", num);

    for i in 1..101 {
        // println!("i: {}", i);
    }
    let x= 5;

    // similar to switch statements
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else")
    }
}
