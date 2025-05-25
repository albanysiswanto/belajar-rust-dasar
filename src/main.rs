fn main() {
    println!("Hello, world!");
    println!("Hello, Bany!");
}

#[test]
fn test_variable() {
    let name = "Albany Siswanto"; // data tidak bisa diubah
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Albany Siswanto"; // data bisa diubah(muttable)
    println!("Hello {}", name);

    name = "Budi Nugraha";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Albany Siswanto";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Albany Siswanto"; // layer 1
    println!("Hello {}", name);

    let name = 10; // layer 2, tandanya layer 1 tertutupi
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let age: i32 = 20; // explicit
    println!("{}", age);

    let age = 30; // inplicit
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

// 1:01:58
