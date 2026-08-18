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
