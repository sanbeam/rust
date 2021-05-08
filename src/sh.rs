#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
use rand::Rng;
use std::io::stdin;

use std::collections::HashMap;
use std::collections::HashSet;

enum State
{
    Locked,
    Failed,
    Unlocked
}

struct Point
{
    x: f64,
    y: f64
}

struct Point1<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point1<T>,
    end: Point1<T>
}

pub fn generics()
{
    let a = Point1{x:0.0, y:4f64};
    let b = Point1{x: 1.2, y: 3.4};

    let line = Line{start: a, end: b};

}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    YUVColor{yellow:u8, ultra:u8, violet:u8},
}

pub fn enums()
{
    let c:Color = Color::YUVColor{yellow: 10,ultra: 255,violet: 8};

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0,0,0) =>println!("black"),
        Color::RgbColor(r,g,b) =>println!("other color {}{}{}", r,g,b),
        Color::YUVColor{yellow: _,ultra: _,violet: 255} =>println!("VIOLET"),
        Color::YUVColor{ultra:255,..} =>println!("ULTRA"),
        _ => ()
    }
}

fn origin() -> Point
{
    Point{x:0.0, y:0.0}
}

pub fn structures()
{
    let p = Point{x:3.0, y:4.0};
    println!("Point P is at {} {}", p.x, p.y);
}



pub fn stack_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes {} bytes", mem::size_of_val(&p1));
    println!("p2 takes {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3 x {} y {}", p3.x, p3.y);
}



pub fn combination_lock()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry==code {
                    state = State::Unlocked;
                }

                if !code.starts_with(&entry){
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Correct");
                return;
            }
        }
    }

}

pub fn templates() {
    let x = 3.0;
    let y = 2.0;

    let result =
        if y!=0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Divide by zero")
    }

    if let Some(z)  = result {
        println!("result = {}", z)
    }

    while let Some(z)  = result {
        println!("result = {}", z);
        break;
    }

}

pub fn array(){
    let a:[i32;5] = [321,2,3,4,5];
    println!("a has {} and first is {}", a.len(), a[0]);
    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("No match");
    }

    let b = [1u8;10];

    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }
    println!("b took {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = [
        [1.0, 2.2, 0.0],
        [2.3, 5.4, 1.2]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len() {
            println!("{}", mtx[i][j]);
        }
    }
}
fn use_slices(slice: &[i32]){
    println!("1{} l{}", slice[0], slice.len());
}

pub fn slices() {
    let data = [1,2,3,4,5];
    use_slices(&data[1..4]);
}

fn sumprod(x:i32, y:i32)->(i32, i32) {
    (x+y, x*y)
}

pub fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sumprod(x, y);
    let sp2 = sumprod(5, 8);

    let combined = (sp, sp2);

    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0}, * {1} = {3}", x,y,sp.0, sp.1);
    println!("{:?}", combined);
}

fn how_many(x:i32) -> &'static str
{
    match x {
        0 => "no",
        1|2 => "one or two",
        12 => "a dozen",
        9...11 => "lots of",
        _ if(x%2 == 0) => "some",
        _ => "a few"
    }

}

pub fn patterns()
{
    for x in 0..12
    {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("xaxis, y = {}", y),
        (x, 0) => println!("yaxis, x = {}", x),
        (_, y) => println!("(?:{})", y)
    }
}

pub fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(43);
    println!("a[0] = {}", a[0]);

    a[0] = 32;
    println!("a[0] = {}", a[0]);

    match a.get(3){
        Some(x) => println!("a[3] = {}", x),
        None => println!("no such element")
    }

    for x in &a {print!("{},", x)}
}

pub fn hashmap(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square hash {} sides", shapes["square".into()]);
    // println!("a square hash {} sides", shapes["pentagon".into()]);

    for (key,value) in &shapes {
        println!("a {} has {} sides", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);


    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 2;
    }
    println!("{:?}", shapes);

}

pub fn hashset()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);
    greeks.insert("delta");
    println!("{:?}", greeks);

    let mut addedv = greeks.insert("vega");
    if addedv {
        println!("Added vega!");
    }

    addedv = greeks.insert("vega");
    if addedv {
        println!("Added vega!");
    }

    addedv = greeks.contains("vega");
    if addedv {
        println!("contains vega!");
    }

    addedv = greeks.remove("vega");
    if addedv {
        println!("removed vega!");
    } else {
        println!("not contains vega!");
    }

    addedv = greeks.remove("vega");
    if addedv {
        println!("removed vega!");
    } else {
        println!("not removed vega!");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();
    let _1_10: HashSet<_> = (1..=10).collect();

    println!(
        "is {:?} is a subset of {:?} = {}",
        _2_8, _1_10, _2_8.is_subset(&_1_10)
    );

    println!(
        "is {:?} is a subset of {:?} = {}",
        _6_10, _1_5, _6_10.is_disjoint(&_1_5)
    );

    println!(
        "{:?} union {:?} = {:?}",
        _2_8, _6_10, _6_10.union(&_2_8)
    );

    println!(
        "{:?} intersection {:?} = {:?}",
        _2_8, _6_10, _6_10.intersection(&_2_8)
    );
}

pub fn iterators()
{
    let mut vec = vec![3,2,1];
    let mut vec2 = vec![1,2,3];


    for x in &vec{
        println!("{}", x);
    }
    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter_mut() {

       *x *= 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() {
        println!("{}", x);
    }

    vec2.extend(vec);
    println!("{:?}", vec2);
    // println!("{:?}", vec);
}

pub fn strings(){

    let s:&'static str = "hello there";
    println!("{}", s);
    // s = "hi there";
    // let h = s[0];
    // println!("{}", h);

    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    for c in s.chars().rev() {
        print!("{} ", c);
    }
    println!();

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }
    println!();

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1
    }
    println!("{}", letters);

    let u:&str = &letters;
    println!("{}", u);

    //concat
    let z = letters + "abc";

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));

    let name = "Sanjeev";
    let greeting = format!("Hello I am {}. Nice to meet you", name);
    println!("{}", greeting);

    let name = "Sanjeev";
    let run = "Run";
    println!("{0}, {1}, {0}", run, name);

    let info = format!(
        "the name is {l}, {f} {l}",
        l = "jeev",
        f = "san"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data} {}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}

pub fn number_guessing()
{
    let number = rand::thread_rng().gen_range(1,101);
    loop {
        println!("Enter the guess ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        }else if guess < number {
                            println!("Your guess is too low");
                        }else if guess > number {
                            println!("Your guess is too high");
                        }else {
                            println!("Correct!!!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input {}", e)
                    }
                }

            }
            Err(_) => {
                continue;
            }
        }

    }
}