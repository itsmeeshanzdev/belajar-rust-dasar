fn main() {
    println!("Hello, world!");

    println!("Hello Shandika");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Shandika David Ardiansyah";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Shandika David Ardiansyah";
    println!("Hello {}", name);

    name = "Shandika David";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Shandika David Ardiansyah";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Shandika David Ardiansyah";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

// scalar type

// integer
#[test]
fn explicit() {
    let age: i32 = 21;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}

#[test]
fn number_convertion() {
    let a: i8 = 10;
    println!("{}", a);

    // convert to i16
    let b: i16 = a as i16;
    println!("{}", b);

    // convert to i32
    let c: i32 = a as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    println!("{}", d);

    // test convert to i8 (would be overflow = 0)
    let e: i8 = d as i8;
    println!("{}", e);
}
