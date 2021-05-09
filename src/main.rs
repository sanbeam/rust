#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
mod sh;

const MEANING_OF_LIFE: u8 = 42;
static Z1: i32 = 15;

fn allowed_data_types() {
    let a: u8 = 123;
    println!("Unsigned {}", a);

    let mut b: i8 = 0;
    println!("signed {}", b);
    b = 12;
    println!("signed {}", b);

    let c = 12323423;
    println!("signed {} and takes {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "signed {} and takes {} bytes {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("{} and takes {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 3600.2934;
    println!("{} and takes {} bytes", e, mem::size_of_val(&e));
    let f: f64 = 0.2934234;
    println!("{} and takes {} bytes", f, mem::size_of_val(&f));

    let g = false;
    println!("{} and takes {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    let mut a = 2 + 3;
    println!("Addition {}", a);
    a = a + 1;
    a -= 2;
    println!("Sub {}", a);

    let a3 = i32::pow(a, 3);
    println!("pow {}", a3);

    let b = 2.5;
    let b3 = f64::powi(b, 3);
    println!("pow {}", b3);
    let b2pi = f64::powf(b, std::f64::consts::PI);
    println!("pow {}", b2pi);

    let c = 1 | 2;
    println!("bitor {}", c);

    let twopow10 = 1 << 10;
    println!("bitor {}", twopow10);

    let pi_less_4 = std::f64::consts::PI < 4.0;
    let x = 5;
    let xis5 = x == 5;
    println!("less {} and eq {}", pi_less_4, xis5);
}

fn scope_and_shadow() {
    let a = 123;

    {
        let b = 456;
        println!("inside b = {}", b);

        let a = 555;
        println!("inside a = {}", a);
    }
    println!("outside a = {}", a);
}

fn consts() {
    print!("{}", MEANING_OF_LIFE);
    print!("{}", Z1);
}

fn if_statements()
{
    let temp = 25;

    if temp > 30
    {
        println!("really hot outside");
    }
    else if temp < 10
    {
        println!("really cold!");
    }
    else {
        println!("temperature is ok");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}",day);
}

fn while_loop()
{
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        println!("x = {}", x);
    }
}

fn for_loops()
{
    for x in 1..11
    {
        println!("x is {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("pos is {} y is {}", pos, y);
    }
}

fn matches()
{
    let cc = 1001;

    let country = match cc{
        82 => "South Korea",
        91 => "India",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("Country {} is {}", cc, country);
}

union IOF {
    i:i32,
    f:f32
}

fn process(iof: IOF){
    unsafe {
        match iof {
            IOF{i:42} => {
                println!("Meaning of life")
            }
            IOF {f} => {
                println!("Float = {}", f);
            }
            // IOF {i} => {
            //     println!("Int = {}", i);
            // }
        }
    }
}

fn main() {
    println!("Hello World");
    // allowed_data_types();
    // operators();
    // scope_and_shadow();
    // consts();

    // sh::stack_heap();

    // if_statements();

    // while_loop();

    // for_loops();

    // matches();

    // sh::combination_lock();

    // sh::structures();

    // sh::enums();
    // let mut iof = IOF{i:123};
    // println!("{}", unsafe{iof.i});
    // process(iof);

    // sh::templates();

    // sh::array();

    // sh::slices();

    // sh::tuples();

    // sh::enums();

    // sh::patterns();

    // sh::generics();

    // sh::vectors();

    // sh::hashmap();

    // sh::hashset();

    // sh::iterators();

    // sh::strings();

    // sh::number_guessing();

    // sh::function();

    // sh::methods();

    // sh::closures();

    // sh::higher_order_functions();

    // sh::traits();

    // sh::trait_params();

    // sh::into_traits();

    sh::droptest();
}









