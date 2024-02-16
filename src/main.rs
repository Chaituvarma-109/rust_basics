mod m10_proc_macros;
mod m11_smart_pointers;
mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;
mod m12_concurrency;

const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course {}!", OUR_COURSE);

    // Stack
    let x: i32 = 2;
    println!("Value of x is {}", x);

    let y: i32 = 5;
    println!("Value of y is {}", y);

    // for loop
    for i in 0..=y {
        if i != 5 {
            print!("{} ", i);
        } else {
            println!("{}", i);
        }
    }

    // Mutation
    let mut z = 10;
    println!("Value of z is {}", z);
    z = 20;
    println!("Value of z is {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing temp is {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("zero temp is {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my char is {}", my_char);

    let emoji: char = '😎';
    println!("my emoji is {}", emoji);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my floats are {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|x| x + 2.0);
    println!("my floats new are {:?}", my_floats_new);

    let name: &str = "AutoGPT";
    println!("my name is {}", name);

    let dynamic_name: String = String::from("Chaitu");
    println!("dynamic name is {}", dynamic_name);
    println!("dynamic name address is {:p}", &dynamic_name);

    let str_slice: &str = &dynamic_name[0..3];
    println!("str slice is {}", str_slice);

    let mut vec: Vec<char> = Vec::new();
    vec.insert(0, 'h');
    vec.insert(1, 'e');
    vec.insert(2, 'l');
    vec.push('l');
    vec.push('o');
    vec.push('.');
    dbg!(&vec);

    let rem_char: char = vec.pop().unwrap();
    dbg!(&rem_char);
    println!("vec char is {:?}", vec);

    vec.iter().for_each(|c| println!("char is {}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect(); // collect::<String>
    dbg!(&collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            println!(", world!");
        }
    }

    // closures
    let num: i32 = 5;
    let add_name = |x: i32| x + num;
    let new_name: i32 = add_name(5);
    dbg!(&new_name);

    // Number literals (from rust book)
    println!("Big Number is {}", 98_222_000);
    println!("Hex Number is {}", 0xff);
    println!("oct Number is {}", 0o77);
    println!("binary Number is {}", 0b1111_0000);
    println!("Bytes for char 'A' is {}", b'A');

    // Raw - String Literal
    let txt: &str = r#"{"hello": "world"}"#;
    dbg!(&txt);
    println!("Raw String is {:p}", *&txt);

    // Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;

    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a's binary value is {:08b}", a);
    println!("b's binary value is {:08b}", b);

    // Logic gates
    println!("{} AND {} is {}", a, b, (a & b));
    println!("{} OR {} is {}", a, b, (a | b));
    println!("{} XOR {} is {}", a, b, (a ^ b));
    println!("NOT {} is {}", a, !a);
    println!("NOT {} is {}", b, !b);

    // Bitwise Operations
    println!("{} << 1 is {:08b}", a, (a << 1));
    println!("{} >> 1 is {:08b}", a, (a >> 1));
    println!("{} << 1 is {:08b}", b, (b << 1));
    println!("{} >> 1 is {:08b}", b, (b >> 1));

    // Little Endian and Big Endian
    println!("Little Endian is {:?}", 0x12345678);
    println!("Big Endian is {:?}", 0x78563412);

    let n: u16 = 0x1234;
    let big_endian: [u8; 2] = n.to_be_bytes();
    let little_endian: [u8; 2] = n.to_le_bytes();

    println!("Big Endian is {:?}", big_endian);
    println!("Little Endian is {:?}", little_endian);
}
