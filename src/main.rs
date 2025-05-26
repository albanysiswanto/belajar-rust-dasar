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

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000; // terjadi integer overflow karena nilai melebihi kapasitas tipe data i8
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 20;
    let b = 20;

    let result: bool = a >= b;
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    // Jika kita membuat Tuple dengan total ada 3 data, maka tidak akan bisa diubah lagi jumlah data dan juga tipe data nya
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // Mengakses data dalam tuple
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    // Destructuring Tuple
    // Jika ada data di Tuple yang tidak kita butuhkan, kita bisa gunakan tanda _ (garis bawah) ketika melakukan destructuring Tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

/*
Tuple kosong
Unit adalah tuple tanpa nilai apapun, ditulisnya ()
Hal ini mungkin kelihatan tidak ada gunanya
Biasanya Unit ini digunakan untuk function-function yang tidak membutuhkan hasil data apapun
 */
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

/*
Array adalah tipe data yang berisi kumpulan data, sama seperti Tuple
Yang membedakan Array dengan Tuple adalah, pada Array, kita hanya bisa menggunakan satu tipe data
Untuk membuat Array, kita bisa gunakan [] tanda kurung kotak
*/
#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5]; // explicit
    println!("{:?}", array);

    // Mengakses array
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    // Hal yang membedakan dengan Tuple, kita bisa mendapatkan jumlah data di Array dengan menggunakan function len() milik Array
    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

/*
Constant adalah variable immutable menggunakan kata kunci const
Yang membedakan const dan let adalah, const tidak memiliki mutable, selain itu nilai const harus dideklarasikan ketika kode program dibuat (bukan dijalankan), oleh karena itu nilai const tidak bisa hasil dari kalkulasi nilai lain yang belum jelas hasilnya
Untuk membuat const, wajib disebutkan tipe datanya secara eksplisit
Nama const di Rust harus huruf besar, dan biasanya pemisah kata menggunakan _ (garis bawah)
*/
const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

/*
Dalam dunia programming, variable scope mendefinisikan area dimana variable bisa digunakan
Variable bisa digunakan di dalam scope tempat definisi variable, dan juga di inner scope
Namun variable tidak bisa digunakan di outer scope
*/
#[test]
fn variable_scope() {
    println!("{}", MAXIMUM);

    let bany = 1;

    {
        println!("{}", bany);
        let siswanto = 2;
        println!("{}", siswanto);
    }

    // println!("{}", siswanto); //error
}
