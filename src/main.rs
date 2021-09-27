fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    //takes_ownership(s1);

    //let x = 5;
    //makes_copy(x);
    //println!("{}", x);

    //let len = calculate_length(&s1);
    //println!("{}", len)

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}
