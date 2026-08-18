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
