mod first;
mod model;
mod second;
mod third;

use crate::first::say_hello;
use crate::second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    first::second::third::say_hello();
}

use model::User;

#[test]
fn test_module() {
    let user: User = User {
        first_name: String::from("Eko"),
        last_name: String::from("Khannedy"),
        username: String::from("khannedy"),
        email: String::from("khannedy@example.com"),
        age: 20,
    };

    user.say_hello("Budi");
}

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

/*
Garbage Collection
Garbage Collection adalah fitur yang banyak digunakan bahasa pemrograman untuk melakukan manajemen memory, seperti Java dan Golang
Secara berkala Garbage Collection akan memantau data yang sudah tidak digunakan lagi di memory, dan menghapusnya secara otomatis
Atau di bahasa pemrograman tanpa Garbage Collection, yang biasanya harus melakukan manajemen memory secara manual, seperti C/C++
Tanpa Garbage Collection, kita harus mengalokasikan data secara manual di memory, begitu juga ketika sudah tidak butuh, kita harus menghapus data dari memory dari memory
Rust memiliki pendekatan yang berbeda, Rust tidak menggunakan Garbage Collection, Rust juga tidak menggunakan Manual Memory Management
*/

/*
Stack dan Heap
Rust membagi data di memory dalam dua bagian, Stack dan Heap
Stack adalah bagian dimana data disimpan dalam struktur data tumpukan, last in first out. Semua data di Stack harus yang fixed size (artinya ukuran data sudah pasti)
Heap berbeda, heap seperti tempat untuk menyimpan data, dimana untuk menyimpan data di Heap kita akan melakukan request ke Heap, lalu di dalam Heap terdapat Memory Allocator yang bertugas untuk menemukan area kosong untuk menyimpan dan mengalokasikan data ke area tersebut. Setelah itu kita akan diberi pointer (penunjuk) ke lokasi dimana data itu berada di Heap.
Pointer dari Heap berukuran fix sized, oleh karena itu pointer akan disimpan di Stack
*/
#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10; // disimpan di stack karena ukuran data sudah pasti
    let b = String::from("Siswanto"); // disimpan di heap karena ukuran data tidak pasti
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10; // disimpan di stack karena ukuran data sudah pasti
    let b = String::from("Albany"); // disimpan di heap karena ukuran data tidak pasti
    println!("{}, {}", a, b);
}

/*
Drop Function
Saat variable keluar dari scope nya, yang artinya tidak bisa diakses lagi, secara otomatis Rust akan memanggil drop function
Drop function adalah function untuk menghapus data, sehingga akan dibersihkan dari Heap
Dan jika Rust function() sudah selesai dieksekusi, maka function() tersebut akan dihapus pula dari Stack Frame
Oleh karena itu, Rust tidak membutuhkan Garbage Collection ataupun Manual Memory Management
*/

/*
&str dan String
Rust memiliki tipe data text yang fixed size, yaitu &str (string slice), dan yang bisa mengembang ukurannya, yaitu String
&str karena ukurannya fixed size, jadi Rust akan menyimpannya di Stack, sedangkan String karena bisa mengembang, maka disimpan di Heap
*/
/*
Immutable str
Karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable, artinya isi data &str tidak bisa diubah
Ketika kita buat variable mutable, dan mengubah data &str, sebenarnya yang dilakukan adalah mengganti isi variable, bukan mengubah isi dari &str
&str memiliki banyak sekali method yang bisa digunakan untuk memanipulasi &str nya, namun akan menghasilkan nilai &str baru
Namun perlu diperhatikan, beberapa method dari &str akan mengembalikan bentuk data String, bukan &str
https://doc.rust-lang.org/std/primitive.str.html
*/
#[test]
fn string() {
    let name: &str = "   Albany Siswanto   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

/*
String
String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya
Ketika kita buat dalam bentuk immutable variable, maka String tidak bisa berkembang, namun tetap disimpan di Heap
Ketika kita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
String juga memiliki method / function untuk memanipulasi data, namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri, ada juga method yang digunakan untuk mengubah dalam bentuk data baru, tanpa memodifikasi data asli nya
https://doc.rust-lang.org/std/string/struct.String.html
*/
#[test]
fn string_type() {
    let mut name: String = String::from("Albany");
    println!("{}", name);

    name.push_str(" Siswanto");
    println!("{}", name);

    let budi = name.replace("Albany", "Budi");
    println!("{}", name);
    println!("{}", budi);
}

/*
Ownership Rules
Setiap value di Rust harus punya owner (variable pemilik value)
Dalam satu waktu, hanya boleh ada satu owner
Ketika owner keluar scope, value akan dihapus
*/
#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

/*
Data Copy (hanya data yang disimpan di stack)
Sesuai aturan di Ownership Rules, setiap value harus dimiliki oleh satu owner pada satu waktu
Ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
Semua data yang bersifat fixed size (yang disimpan di Stack), ketika kita tambahkan ke variable berbeda (owner baru), maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data hasil copy dari variable lama (owner lama)
Oleh karena itu, tiap data akan selalu dimiliki oleh satu owner pada satu waktu
*/
#[test]
fn data_copy() {
    let a = 10;
    let b = a; // copy data dari a ke b (copy data dari owner lama ke owner baru)

    println!("{} {}", a, b);
}

/*
Ownership Movement (hanya data yang disimpan di heap)
Namun Data Copy tidak terjadi untuk tipe data yang disimpan di Heap
Seperti aturan di Ownership, dalam satu waktu value hanya dimiliki satu owner
Maka ketika kita coba buat variable baru (owner baru) dari variable lama (owner lama), maka yang terjadi bukanlah copy, melainkan transfer ownership dari owner lama ke owner baru
Setelah proses transfer selesai, secara otomatis owner lama akan dianggap tidak valid lagi digunakan
*/
#[test]
fn ownership_movement() {
    let name1: String = String::from("Bany");
    println!("{}", name1);

    let name2: String = name1; // ownerhsip pindah ke name2
    println!("{}", name2);
    // println!("name {}", name1);
}

/*
Clone
Sekarang kita tahu bahwa data di Stack akan di Copy sedangkan data di Heap akan dipindahkan ownership nya
Lantas bagaimana jika kita juga ingin melakukan Copy untuk data di Heap?
Maka kita harus melakukan Clone
Clone adalah membuat data tiruan yang sama dari data aslinya
String memiliki method clone() untuk melakukan ini
Saat kita memanggil method clone() maka method tersebut akan meng-copy data String menjadi data String baru
Semua tipe data yang disimpan di Heap di Rust memiliki method clone()
*/
#[test]
fn clone() {
    let name1 = String::from("Bany");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

/*
Let Statement
If di Rust adalah sebuah expression, artinya bisa menghasilkan value dan bisa digunakan dengan Let statement untuk mengisi data di variable
Ini sangat berguna sehingga kita tidak perlu memasukkan nilai ke variable terpisah dengan deklarasi variable nya
*/
#[test]
fn if_expression() {
    let value = 10;
    // Let Statement
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

/*
Range
Rust memiliki tipe data bernama Range
Range adalah jarak antara start dan end
Range merupakan tipe data Collection seperti Array, sehingga bisa dilakukan pengulangan menggunakan For Loop
Data range akan dimulai dari start dan diakhiri sebelum end (exclusive)
https://doc.rust-lang.org/std/ops/struct.Range.html
*/
#[test]
fn range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

/*
Range Inclusive
Selain Range yang exclusive, Rust juga memiliki tipe data Range Inclusive
Implementasinya berbeda dengan Range sebelumnya
https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
*/
#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

// fn say_hello() {
//     println!("Hello");
// }

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Albany", "Siswanto");
    say_goodbye("Budi", "Nugraha");
    say_goodbye("Joko", "Susilo");
}

/*
Saat membuat function, kadang kita ingin mengembalikan hasil eksekusi yang dilakukan di dalam function, atau kita sebut Return Value
Jika sebuah function ingin mengembalikan value, kita bisa sebutkan ketika deklarasi function menggunakan tanda -> lalu diikuti dengan tipe data kembalian value nya
Baris eksekusi terakhir di function akan dianggap sebagai kembalian value-nya
Atau jika kita ingin mengembalikan value sebelum baris eksekusi terakhir, kita bisa gunakan kata kunci return, dan diikuti dengan value yang akan dikembalikan
*/
fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);

    let result: i32 = factorial_loop(-10);
    println!("{}", result);
}

/*
Recursion merupakan salah satu metode pemecahan masalah dimana sebuah solusi pada masalah tersebut bergantung pada solusi dari masalah yang lebih kecil yang merupakan bagian dari masalah tersebut.
Rust mengimplementasikan recursion dengan memperbolehkan sebuah fungsi untuk memanggil dirinya sendiri (fungsi itu sendiri).
Fungsi yang memanggil fungsi itu sendiri biasanya disebut dengan Recursive Function
Misal kita akan buat dua contoh kasus, pertama kita akan melakukan println tulisan sebanyak parameter menggunakan recursive function. Kedua kita akan ubah factorial sebelumnya menjadi recursive function
*/
fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Bany"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

// stack
fn print_number(number: i32) {
    println!("number {}", number);
}

// heap
fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number); // print_number(10)
    println!("{}", number);

    let name = String::from("Eko");
    hi(name);
    // println!("{}", name);
}

/*
Return Value Ownership
Seperti yang sudah kita tahu, bahwa function bisa mengembalikan value
Value Heap yang kita kembalikan di function, secara otomatis ownership nya akan dimiliki oleh yang memanggil function tersebut
Sedangkan jika Value Stack, maka return value function akan di copy oleh yang memanggil function nya
*/
fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Albany");
    let last_name = String::from("Siswanto");

    let full_name = full_name(&first_name, &last_name); // Menggunakan References

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

/*
Borrowing
Ketika kita membuat reference, aksi itu kita sebut borrowing (meminjam).
Kalo diibaratkan di kehidupan, kita bisa meminjam barang, tapi jika sudah selesai menggunakan barang nya, kita harus mengembalikan ke owner (pemilik) barang nya
Saat kita mencoba memodifikasi value dari reference, maka secara default, hal itu tidak bisa dilakukan, jadi secara default reference adalah immutable, walaupun variable owner nya sendiri adalah mutable
*/
fn change_value(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Eko");

    let value_borrow = &mut value; // Mutable Reference (ownernya harus mutable)

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);

    println!("{}", value);
}

/*
Solusi Dangling Pointer
Jika menang data yang dikembalikan dibuat di dalam function, maka kita harus kembalikan dalam bentuk value langsung, bukan reference
Atau kita bisa mengeluarkan variable owner dari value diluar function, agar masuk variable scope, sehingga Rust tidak menghapus variable dan value tersebut setelah function selesai di eksekusi
*/
fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Albany");
    let last_name = String::from("Siswanto");

    let full_name = get_full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Albany Siswanto");

    let first_name: &str = &name[0..=5];
    println!("{}", first_name);

    let last_name: &str = &name[6..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Eko");
    let last_name = String::from("Khannedy");

    let person: Person = Person {
        age: 20,
        first_name,
        middle_name: String::from("Kurniawan"),
        last_name,
    };

    print_person(&person);

    let person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person // Memindahkan Ownership(Update Syntax di Struct)
    };

    print_person(&person2);

    println!("{}", person.first_name);
}

struct GeoPoint(f64, f64);

// Associated Functions
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(10.0, 10.0);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.23223, 100.23232);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing; // Jika tidak niat digunakan tambahkan _(garis bawah) agar tidak memunculkan warning
    let _nothing2: Nothing = Nothing {};
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Eko"),
        last_name: String::from("Eko"),
        age: 20,
    };

    person.say_hello("Budi");

    println!("{}", person.first_name);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level: Level = Level::Premium;
    // let _level2: Level = Level::Regular;
    // let _level3: Level = Level::Platinum;

    // Pattern Matching Enum
    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with ewallet {} {} amount {}",
                    wallet, number, amount
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("423423432"));
    _payment1.pay(500000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("32423423"));
    _payment2.pay(3423423);

    let _payment3: Payment = Payment::EWallet(String::from("Gopay"), String::from("32423423"));
    _payment3.pay(3453543);
}

// Test Match with value
#[test]
fn test_match_value() {
    let name = "Bany";

    match name {
        "Eko" => {
            println!("Hello Eko");
        }
        "Budi" => {
            println!("Hello Budi");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Eko" | "Budi" | "Joko" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 99;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        other => {
            println!("Invalid value {}", other)
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {}, lat : {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Kurniawan"),
        last_name: String::from("Khannedy"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("long : {}", long);
        }
    }
}

#[test]
fn test_ignoring_range() {
    let value = 67;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        _ => {
            println!("Invalid value")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 9;

    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid",
    };

    println!("{}", result);
}

// Type Alias
type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

// type Pelanggan = Customer;

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("324234234"),
        name: String::from("Eko"),
        age: 20,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Eko"),
        last_name: String::from("Eko"),
        age: 20,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let result = person.say_hello_to("Budi");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Budi"));

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Budi");
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Eko"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Budi"));
}

// Super Trait(atau pewarisan di basaha berorientasi lain)
trait CanSay: CanSayHello + CanSayGoodBye {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }
}

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let integer: Point<i32> = Point::<i32> { x: 1, y: 2 };
    let float: Point<f64> = Point::<f64> { x: 1.0, y: 2.0 };

    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value: Value<i32> = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

struct Hi<T = SimplePerson>
where
    T: CanSayGoodBye,
{
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Eko"),
        },
    };
    println!("{}", hi.value.name);
}

fn min<T>(value1: T, value2: T) -> T
where
    T: PartialOrd,
{
    if value1 < value2 { value1 } else { value2 }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);

    let result = min(20.0, 10.0);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point { x: 10, y: 20 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

use core::ops::Add;

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}
