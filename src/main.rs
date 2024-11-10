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
