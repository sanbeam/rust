#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
use rand::Rng;
use std::io::stdin;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};

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

struct Line1 {
    start: Point,
    end: Point
}

impl Line1 {

    fn length(&self) {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        let len = ((dx * dx) + (dy * dy)).sqrt();
        println!("{} Len of Line", len);
    }

}

pub fn generics()
{
    let a = Point1{x:0.0, y:4f64};
    let b = Point1{x: 1.2, y: 3.4};
    let _line = Line{start: a, end: b};
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
        9..=11 => "lots of",
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
    let _z = letters + "abc";

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

fn print_value(x:i32) {
    println!("Value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y:i32) -> i32 {
    x * y
}

pub fn function() {
    print_value(32);
    let mut z = 1;
    increase(&mut z);
    print_value(z);

    print_value(product(3,5));
}

pub fn methods(){
    let a = Point{x:0.0, y:4f64};
    let b = Point{x: 1.2, y: 3.4};
    let line = Line1{start: a, end: b};
    line.length();
}

pub fn closures() {
    let plus_one = |x:i32| -> i32 {x + 1};
    let a = 6;
    println!("{} + {} = {}", a, 1, plus_one(a));

    let mut seven = 7;
    {
        let plus_seven = |x| -> i32
            {
                let mut z = x;
                z += seven;
                z
            };
        println!("{} + {} = {}", a, seven, plus_seven(a));
    }

    let _borrow_7 = &mut seven;

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    let plus_five = |mut _x: i32| _x += 5;
    let e = 12;
    plus_five(e);
    println!("e = {}", e);
}

fn is_even(x:u32) -> bool {
    x %2 == 0
}

fn gt(limit: u32)
    -> impl Fn(u32) -> bool
{
    move |y| y > limit
}

pub fn higher_order_functions() {

    //functions that take or return functions
    let limit = 500;
    let mut sum = 0;

    let al = gt(limit);

    for i in 0.. {
        let isq = i*i;

        if al(isq) {
            break;
        }
        else if is_even(isq){
            sum += isq;
        }
    }
    println!("loop sum {}", sum);


    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum+x);

    println!("loop sum {}", sum2);

}

trait Animal
{
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human {name: name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat {name: name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self { result += *x;}
        return result;
    }
}


pub fn traits()
{
    // let h = Human{name: "Sanjeev"};
    // let h = Human::create("Sanjeev");
    let h:Human = Animal::create("Sanjeev");
    h.talk();
    // let c = Cat{name: "Pussy"};
    let c = Cat::create("Pussy");
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());

}

#[derive(Debug)]
struct Circle {
    radius: f64
}
#[derive(Debug)]
struct Square {
    side: f64
}
trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// fn print_area(shape: impl Shape + Debug)
// fn print_area<T: Shape + Debug>(shape: T)
fn print_area<T>(shape: T)
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

pub fn trait_params()
{
    let c = Circle{radius: 98f64};
    print_area(c);
}

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person {name: name.to_string()}
    // }

    // fn new<S: Into<String>>(name: S) -> Person {
    //     Person {name: name.into()}
    // }

    fn new<S>(name: S) -> Person
        where S: Into<String>
    {
        Person {name: name.into()}
    }

}

pub fn into_traits()
{
    //Into
    let _john = Person::new("John");

    let name: String = "Jane".to_string();
    // let jane = Person::new(name.as_ref());
    let _jane = Person::new(name);
}



struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature{name : name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} leaves the game", self.name);
    }
}

pub fn droptest() {
    let clever: Creature;
    {
        let goblin = Creature::new("Jeoff");
        println!("Game proceeds");
        clever = goblin;
        println!("end of scope");
    }
    println!("{}", clever.name);
}

#[derive(Debug)]
struct Complex<T>
{
    re: T,
    im: T

}

impl<T> Complex<T> {
    fn new(re: T, im: T) ->Complex<T> {
        Complex::<T> {re, im}
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;
    //a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}


impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    //a+=b
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T: Neg<Output = T>
{
    type Output = Complex<T>;
    //-a
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re==rhs.re && self.im==rhs.im
    }
}

impl<T: Eq> Eq for Complex<T> where T: Eq {}

pub fn opoverload()
{
    let  a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);

    println!("{:?}", a);
    println!("{:?}", b);

    // println!("{:?}", a + b);

    // a += b;
    // println!("{:?}", a);

    // println!("{:?}", -a);


    println!("{}", a==a);

}

trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String
    {
        format!("i32 = {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String
    {
        format!("String = {}", *self)
    }
}

fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format())
}
//we get two functins here
// the decision on which one to call
// happens at compiletime.


fn print_it_2(z: &Printable)
{
    println!("{}", z.format());
}


pub fn static_dispatch()
{
    let a = 123;
    let b = "Hello".to_string();

    println!("{}", a.format());
    println!("{}", b.format());

    // print_it(a);
    // print_it(b);

    print_it_2(&a);
    print_it_2(&b);

}

pub fn dyn_dispatch()
{
    let shapes:[&Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 2.0},
        &Circle{radius: 3.0},
        &Square{side: 4.0}
    ];

    for (i,shape) in shapes.iter().enumerate()
    {
        println!("Area of #{} is {}", i, shape.area());
    }
}

