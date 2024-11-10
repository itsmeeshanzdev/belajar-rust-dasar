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
    let mut name = "Shandika David Ardiansyah";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}
