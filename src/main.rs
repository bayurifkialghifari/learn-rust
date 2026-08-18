fn main() {
    println!("Hello, world!");

    print!("P");

    print!(", Siang Semua");

    println!(" Belajar Rust")
}

#[test]
fn hello_test() {
    println!("Hello, Test");
}

#[test]
fn variable_immutable_test() {
    let name = "Orang Tampan";

    // name = "GGWP"; Variable ga bisa di rubah dia immutable

    println!("Hello {}", name);
}

#[test]
fn variable_mutable_test() {
    let mut name = "Orang Tampan";

    println!("Hello {}", name);

    name = "Asep Galon";

    println!("Hello {}", name);
}

#[test]
fn static_type_test() {
    let name = "Orang Tampan";

    println!("Hello {}", name);

    // name = 100; // Diawal string gabisa tiba tiba ganti jadi integer

    // println!("Hello {}", name);
}

#[test]
fn shaddowing_test() {
    let name = "Orang Tampan";

    println!("Hello {}", name);

    // Bukan best practice, nama variable bagus nya jangan sama

    let name = 100;

    println!("Hello {}", name);
}

#[test]
fn data_type_test() {
    /*
     * Scalar type
     * integer, float, boolean, char, string
     *
     * Compound type
     * array, tuple, struct, enum
     */
    let phi: f64 = 3.14;
    let unsigned_int: u32 = 100;
    let normal_int: i32 = -100;
    let name: &str = "Name";
    let bool: bool = true;
    let char: char = 'a';

    println!("phi: {}", phi);
    println!("unsigned_int: {}", unsigned_int);
    println!("normal_int: {}", normal_int);
    println!("name: {}", name);
    println!("bool: {}", bool);
    println!("char: {}", char);
}

#[test]
fn data_type_conversion_test() {
    let i8: i8 = 10;
    let i16: i16 = i8 as i16;
    let i32: i32 = i16 as i32;
    let string: &str = "300";
    let i32_from_string: i32 = string.parse().unwrap();

    println!("i8: {}", i8);
    println!("i16: {}", i16);
    println!("i32: {}", i32);
    println!("i32_from_string: {}", i32_from_string);
}
