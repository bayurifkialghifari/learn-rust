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

#[test]
fn numeric_operations_test() {
    let a = 10;
    let b = 20;
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let sisa_hasil_bagi = b % a;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("sisa_hasil_bagi: {}", sisa_hasil_bagi);
}

#[test]
fn augmented_assignment_test() {
    let mut a = 10;

    println!("a: {}", a);

    a += 10;

    println!("a: {}", a);

    a -= 5;

    println!("a: {}", a);

    a *= 2;

    println!("a: {}", a);

    a /= 3;

    println!("a: {}", a);

    a %= 2;

    println!("a: {}", a);
}

#[test]
fn boolean_operations_test() {
    let mut a = true;

    println!("a: {}", a);

    a = !a;

    println!("a: {}", a);
}

#[test]
fn comparison_operations_test() {
    let a = 10;
    let b = 20;

    println!("a: {}", a);
    println!("b: {}", b);

    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a < b: {}", a < b);
    println!("a > b: {}", a > b);
    println!("a <= b: {}", a <= b);
    println!("a >= b: {}", a >= b);
}

#[test]
fn tupple_test() {
    // Tuple itu kaya array tapi bisa menyimpan tipe data yang berbeda

    let t = (10, "GGWP");
    println!("t: {:?}", t); // :? adalah format string untuk menampilkan tuple
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    let (a, b) = t;
    println!("a: {}", a);
    println!("b: {}", b);

    let tupple: (i32, &str) = (a, b); // Explicitly annotate the type
    println!("tupple: {:?}", tupple);

    // mutable tuple
    let mut tupple_mut = tupple;
    tupple_mut.0 = 20;
    println!("tupple_mut: {:?}", tupple_mut);
}

// fn unit() {
//     println!("Hello Unit");
// }

#[test]
fn tupple_unit() {
    // let result: () = unit();
    let result = ();
    println!("result: {:?}", result);
}

#[test]
fn array_test() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // panjang nya ga dinamis
    println!("arr: {:?}", arr);
    println!("arr 0: {}", arr[0]);
    println!("arr 1: {}", arr[1]);
    println!("arr 2: {}", arr[2]);
    println!("arr 3: {}", arr[3]);
    println!("arr 4: {}", arr[4]);

    // Mutable array
    let mut arr_mut = arr;
    arr_mut[0] = 10;
    arr_mut[1] = 20;
    arr_mut[2] = 30;
    arr_mut[3] = 40;
    arr_mut[4] = 50;
    println!("arr_mut: {:?}", arr_mut);

    let panjang = arr_mut.len();
    println!("panjang: {}", panjang);
}

#[test]
fn two_dimensional_array_test() {
    let matrix: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("matrix: {:?}", matrix);
    println!("matrix 0: {:?}", matrix[0]);
    println!("matrix 1: {:?}", matrix[1]);
    println!("matrix 2: {:?}", matrix[2]);

    println!("matrix 0.0: {}", matrix[0][0]);
    println!("matrix 0.1: {}", matrix[0][1]);
    println!("matrix 0.2: {}", matrix[0][2]);
    println!("matrix 1.0: {}", matrix[1][0]);
    println!("matrix 1.1: {}", matrix[1][1]);
    println!("matrix 1.2: {}", matrix[1][2]);
    println!("matrix 2.0: {}", matrix[2][0]);
    println!("matrix 2.1: {}", matrix[2][1]);
    println!("matrix 2.2: {}", matrix[2][2]);

    let mut matrix_mut = matrix;

    matrix_mut[0][0] = 10;
    matrix_mut[0][1] = 20;
    matrix_mut[0][2] = 30;
    matrix_mut[1][0] = 40;
    matrix_mut[1][1] = 50;
    matrix_mut[1][2] = 60;
    matrix_mut[2][0] = 70;
    matrix_mut[2][1] = 80;
    matrix_mut[2][2] = 90;
    println!("matrix_mut: {:?}", matrix_mut);
}

#[test]
fn const_test() {
    // Constan harus langsung dikasih tipe data
    const PI: f64 = 3.14;

    println!("PI: {}", PI);
}

/*
 * Stack vs Heap
 *
 * Stack: int, float, char, bool, string, function, pointer itu semua masuk ke stack, stack itu sesuatu yang nilai nya udah fixed
 *
 * Heap: kaya gudang, buat semua yang nilainya belum fixed, kaya String, Vec, HashMap ini masuk ke heap soal nya string nya bisa ngurang bisa nambah
 *
 * Heap kalau udah keluar scope, maka data nya akan dihapus via trait Drop
 *
 */
#[test]
fn stack_heap_test() {
    heap_stack_a();
    heap_stack_b();
}

fn heap_stack_a() {
    let a = 10; // masuk ke stack
    let b = 9.4; // masuk ke stack
    let c = String::from("qwe"); // masuk ke heap

    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn heap_stack_b() {
    let a = "qweqweqwe"; // masuk ke stack
    let b = String::from("qwe"); // masuk ke heap
    println!("a: {}, b: {}", a, b);
}

#[test]
fn string_vs_str_test() {
    let name = " qwe ";
    let trim = name.trim(); // ini return nya &str

    println!("name: {}", name);
    println!("trim: {}", trim);

    let mut string: String = String::from("QWEQWE");
    string.push_str(" 123123"); // Harus mutable
    let new_string = string.replace("QWE", "123"); // Ini bakal ngebuat heap baru, jadi harus diassign ke variabel baru

    println!("string: {}, new_string: {}", string, new_string);
}

#[test]
fn ownership_rules_test() {
    // Basic semua juga sama

    // a gabisa di akses sebelum diinisialisasi
    let a = 10; // a bisa diakses

    {
        // b gabisa di akses
        let b = 20; // b bisa diakses
        println!("b: {}", b);
    } // b gabisa diakses setelah keluar scope

    println!("a: {}", a);
} // ga bisa akses a, udah keluar scope

#[test]
fn data_copy_ownership_movement_test() {
    let a = 100;
    let b = a; // b adalah copy dari a, jadi bisa diakses setelah keluar scope

    println!("a: {}, b: {}", a, b);

    // Kalau heap ga bisa kaya diatas
    let a = String::from("qwe");
    let b = a; // a udah pindah ownership ke b

    // println!("a: {}, b: {}", a, b); // Bakal error, a udah pindah ownership ke b

    println!("a ga ada, b: {}", b);
}

#[test]
fn clone_test() {
    let a = String::from("qwe");
    let b = a.clone(); // b adalah copy dari a, jadi bisa diakses setelah keluar scope

    println!("a: {}, b: {}", a, b);
}

#[test]
fn if_expression_test() {
    // Sama aja kaya bahasa pemrograman lain
    let nilai = 87;
    let hasil: char;

    if nilai >= 90 {
        hasil = 'A';
    } else if nilai >= 80 {
        hasil = 'B';
    } else {
        hasil = 'C';
    }

    println!("nilai: {}, hasil: {}", nilai, hasil);

    // Cara 2
    let hasil2 = if nilai >= 90 {
        'A'
    } else if nilai >= 80 {
        'B'
    } else {
        'C'
    };

    println!("nilai: {}, hasil2: {}", nilai, hasil2);
}

/*
 *
 * Loop di rust ada loop, while, for
 * Bisa di break dan continue
 *
 */
#[test]
fn loop_test() {
    let mut i = 0;

    loop {
        i += 1;
        if i == 10 {
            break;
        }
    }

    println!("i: {}", i);
}

#[test]
fn continue_test() {
    let mut i = 0;

    loop {
        i += 1;

        if i % 2 == 0 {
            continue;
        }

        if i > 10 {
            break;
        }

        println!("i: {}", i);
    }
}

#[test]
fn loop_return_test() {
    let mut i = 0;
    let res = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };

    println!("res: {}", res);
}

#[test]
fn while_test() {
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("i: {}", i);
    }
}

#[test]
fn for_test() {
    for i in 0..10 {
        println!("i: {}", i);
    }
}

#[test]
fn loop_label_test() {
    'outer: loop {
        for i in 0..10 {
            if i == 5 {
                break 'outer;
            }
            println!("i: {}", i);
        }
    }
}

#[test]
fn array_iteration_test() {
    let arr = [1, 2, 3, 4, 5];

    // Pake while
    let mut i = 0;
    while i < arr.len() {
        println!("index: {}, value: {}", i, arr[i]);
        i += 1;
    }

    // Pake for
    for (index, value) in arr.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    for value in arr {
        println!("value: {}", value);
    }

    // Range
    for i in 0..10 {
        println!("i: {}", i);
    }
}

fn say_something(something: &str) {
    println!("{}", something);
}

fn return_function(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    return n * factorial(n - 1); // recursive
}

#[test]
fn function_test() {
    say_something("Hello, World!");

    let res = return_function(1, 2);
    println!("res: {}", res);

    let fact = factorial(5);
    println!("fact: {}", fact);
}

/*
 *
 * Function ownership
 *
 */
fn print_number(num: i32) {
    println!("num: {}", num);
}

fn print_string(name: String) {
    println!("name: {}", name);
}

#[test]
fn ownership_function_test() {
    let num = 5;
    print_number(num);
    println!("{}", num); // Ga error, num nya di copy ke print_number

    let name = String::from("John");
    print_string(name);
    // println!("{}", name); // Bakal error, name sudah dipindahkan ke print_string
    // Kalau mau tetep jalan harus print_string(name.clone())
}

fn full_name(first_name: String, last_name: String) -> String {
    return format!("{} {}", first_name, last_name);
}

fn full_name_with_return_ownership(
    first_name: String,
    last_name: String,
) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    return (full_name, first_name, last_name);
}

#[test]
fn return_value_function_ownership_test() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let name = full_name(first_name, last_name);

    println!("name: {}", name);
    //     println!("first_name: {}", first_name); // Bakal error udah pindah owership ke function full_name
    //     println!("last_name: {}", last_name);
}

#[test]
fn full_name_with_return_ownership_test() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let (full_name, first_name, last_name) = full_name_with_return_ownership(first_name, last_name);

    println!("full_name: {}", full_name);
    println!("first_name: {}", first_name);
    println!("last_name: {}", last_name);
}

/*
 *
 * Reference: ada & mirip kaya di go
 * reference itu immutable
 * kalau ada variable let mut c, reference nya ga bisa diubah
 *
 */
fn full_name_with_reference(first_name: &String, last_name: &String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
fn full_name_with_reference_test() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let full_name = full_name_with_reference(&first_name, &last_name);

    println!("full_name: {}", full_name);
    println!("first_name: {}", first_name);
    println!("last_name: {}", last_name);
}

/*
 *
 * Bakal error ini walaupun si name nya mutable
 * reference, so it cannot be borrowed as mutable
 *
 */
// fn change_reference(name: &String) {
//     name.push_str("qweqwe");
// }

// #[test]
// fn change_reference_test() {
//     let mut name = String::from("John");
//     change_reference(&name);
//     println!("name: {}", name);
// }

/*
 *
 * Cara bisa mutable reference
 *
 * di 1 waktu cuma bisa ada 1 mutable reference
 *
 */
fn change_reference(name: &mut String) {
    name.push_str("qweqwe");
}

#[test]
fn change_reference_test() {
    let mut name = String::from("John");
    change_reference(&mut name);
    println!("name: {}", name);
}

/*
 *
 * Dangling pointer
 *
 * Buat fix ini, langsung return value nya aja
 *
 */
fn dangling_pointer() -> &String {
    // 1. Variabel `teks` dibuat di memori
    let teks = String::from("Halo Dunia");

    // 2. Kita mengembalikan referensi (alamat) dari `teks`
    return &teks;
} // 3. Variabel teks otomatis dihapus dari memori!

#[test]
fn dangling_pointer_test() {
    // pasti error
    let alamat = dangling_pointer();
    // alamat ga ada isi, soal nya si let teks itu udah dihapus

    println!("alamat: {}", alamat);
}
