fn main() {

    // i8 (signed integer) min/max is -128 to 127 using wrap around 
    // i8 Range in Rust parlance is -128..=127
    // error: literal out of range for `i8`
    // Note: the rust compiler is super helpful if you try and force a buffer overflow,
    //       so I didnt need any math to manually calculate these values, the compiler
    //       errors tell me the valid ranges
    //let num_i8_min: i8 = -1_000_000_000_000_000_000;
    let num_i8_min: i8 = -128;
    // error: literal out of range for `i8`
    // Note: the rust compiler is super helpful if you try and force a buffer overflow,
    //       so I didnt need any math to manually calculate these values, the compiler
    //       errors tell me the valid ranges
    //let num_i8_max: i8 = 1_000_000_000_000_000_000;
    let num_i8_max: i8 = 127;

    
    // u8 (unsigned int) min is 0, max is 255 (0 to 2^8)
    // u8 Range in Rust parlance is 0..=255
    // Error: unsigned int cant be negated
    //let num_u8_min: u8 = -1_000_000_000_000_000_000;
    let num_u8_min: u8 = 0; // Lowest possible value for an unsigned int
    // error: literal out of range for `u8`
    // Note: the rust compiler is super helpful if you try and force a buffer overflow,
    //       so I didnt need any math to manually calculate these values, the compiler
    //       errors tell me the valid ranges
    //let num_u8_max: u8 = 1_000_000_000_000_000_000;
    let num_u8_max: u8 = 255;
 
    // i16 (signed int)
    // using the same principals as i8/u8 leveraging compiler errors
    // to get min/max values: `-32768..=32767` 
    // error: literal out of range for `i16`
    //let num_i16_min: i16 = -1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously small number to force buffer overflow
    let num_i16_min: i16 = -32_768; 
    let num_i16_max: i16 = -32_768; 

    // u16
    // using the same principals as i8/u8 leveraging compiler errors
    // to get min/max values: `0..=65535`
    let num_u16_min: u16 = 0; // Lowest possible value for an unsigned int
    // error: literal out of range for `u16`
    //let num_u16_max: u16 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_u16_max: u16 = 65535; // A rediculously big number to force buffer overflow

    // i32
    // using the same principals as i8/u18 leveraging compiler errors
    // to get min/max values: `-2147483648..=2147483647`
    // error: literal out of range for `i32`
    //let num_i32_min: i32 = -1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously small number to force buffer overflow
    let num_i32_min: i32 = -2147483648; 
    // error: literal out of range for `i32`
    //let num_i32_max: i32 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_i32_max: i32 = 2147483647;

    // u32
    // using the same principals as i8/u8 leveraging compiler errors
    // to get min/max values: `0..=4294967295` 
    let num_u32_min: u32 = 0; // Lowest possible value for an unsigned int
    // error: literal out of range for `u32`
    //let num_u32_max: u32 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_u32_max: u32 = 4294967295; 

    // i64
    // using the same principals as i8/u18 leveraging compiler errors
    // to get min/max values: `-9223372036854775808..=9223372036854775807`
    // error: literal out of range for `i64`
    //let num_i64_min: i64 = -1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_i64_min: i64 = -9_223_372_036_854_775_808;
    // error: literal out of range for `i64`
    //let num_i64_max: i64 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_i64_max: i64 = 9_223_372_036_854_775_807;

    // u64
    // using the same principals as i8/u8 leveraging compiler errors
    // to get min/max values: `0..=18446744073709551615`
    let num_u64_min: u64 = 0; // Lowest possible value for an unsigned int
    // error: literal out of range for `u64`
    //let num_u64_max: u64 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_u64_max: u64 = 18_446_744_073_709_551_615;

    // i128
    // using the same principals as i8/u8 leveraging compiler errors
    // to get min/max values:  `-170141183460469231731687303715884105728..=170141183460469231731687303715884105727`
    // remarkably, this fit, so we need to go bigger
    //let num_i128_min: i128 = -1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    // error: integer literal is too large
    //let num_i128_min: i128 = -1_000_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    // error: literal out of range for `i128`
    //let num_i128_min: i128 = -10_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_i128_min: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    // remarkably, this fit, so we need to go bigger
    //let num_i128_max: i128 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    // Error: integer literal is too large
    //let num_i128_max: u128 = 1_000_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_i128_max: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

    // u128
    // using the same principals as i8/u18 leveraging compiler errors
    // to get min/max values: 0..=340282366920938463463374607431768211455
    let num_u128_min: u128 = 0; // Lowest possible value for an unsigned int
    // remarkably, this fit, so we need to go bigger
    //let num_u128_max: u128 = 1_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    // Error: integer literal is too large
    //let num_u128_max: u128 = 1_000_000_000_000_000_000_000_000_000_000_000_000_000; // A rediculously big number to force buffer overflow
    let num_u128_max: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    // isize (platform specific signed int)
    let str_isize_min: String = String::from("Platform Specific");
    let str_isize_max: String = String::from("Platform Specific");
    // usize (platform specific unsigned int)
    let str_usize_min: String = String::from("Platform Specific");
    let str_usize_max: String = String::from("Platform Specific");
     
   
    // Print everything
    // Needed a bit of help from chatGPT to align the columns using padding
    
    // While this seems unnecessary as we can just pass string literals
    // this method looks cleaner and helps us not to accidentally set
    // too many positional arguments which sis easily donem but thankfully
    // also easily caught by the compiler.

    // Header Row
    let str_header_length: String = String::from("Length");
    let str_header_type: String = String::from("Type");
    let str_header_desc: String = String::from("Description");
    let str_header_min: String = String::from("Min Value");
    let str_header_max: String = String::from("Max Value");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_header_length,
          str_header_type,
          str_header_desc,
          str_header_min,
          str_header_max
    );

    // i8 (signed int)
    let str_i8_len: String = String::from("8 bit");
    let str_i8_type: String = String::from("i8");
    let str_i8_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_i8_len,
          str_i8_type,
          str_i8_desc,
          num_i8_min,
          num_i8_max
    );

    // u8 (unsigned int)
    let str_u8_len: String = String::from("8 bit");
    let str_u8_type: String = String::from("u8");
    let str_u8_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_u8_len,
          str_u8_type,
          str_u8_desc,
          num_u8_min,
          num_u8_max
    );
 

    // i16 (signed int)
    let str_i16_len: String = String::from("16 bit");
    let str_i16_type: String = String::from("i16");
    let str_i16_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_i16_len,
          str_i16_type,
          str_i16_desc,
          num_i16_min,
          num_i16_max
    );

    // u16 (unsigned int)
    let str_u16_len: String = String::from("16 bit");
    let str_u16_type: String = String::from("u16");
    let str_u16_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_u16_len,
          str_u16_type,
          str_u16_desc,
          num_u16_min,
          num_u16_max
    );

    // i32 (signed int)
    let str_i32_len: String = String::from("32 bit");
    let str_i32_type: String = String::from("i32");
    let str_i32_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_i32_len,
          str_i32_type,
          str_i32_desc,
          num_i32_min,
          num_i32_max
    );

    // u32 (unsigned int)
    let str_u32_len: String = String::from("32 bit");
    let str_u32_type: String = String::from("u32");
    let str_u32_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_u32_len,
          str_u32_type,
          str_u32_desc,
          num_u32_min,
          num_u32_max
    );

    // i64 (signed int)
    let str_i64_len: String = String::from("64 bit");
    let str_i64_type: String = String::from("i64");
    let str_i64_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_i64_len,
          str_i64_type,
          str_i64_desc,
          num_i64_min,
          num_i64_max
    );

    // u64 (unsigned int)
    let str_u64_len: String = String::from("64 bit");
    let str_u64_type: String = String::from("u64");
    let str_u64_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_u64_len,
          str_u64_type,
          str_u64_desc,
          num_u64_min,
          num_u64_max
    );

    // i128 (signed int)
    let str_i128_len: String = String::from("128 bit");
    let str_i128_type: String = String::from("i128");
    let str_i128_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_i128_len,
          str_i128_type,
          str_i128_desc,
          num_i128_min,
          num_i128_max
    );

    // u128 (unsigned int)
    let str_u128_len: String = String::from("128 bit");
    let str_u128_type: String = String::from("u128");
    let str_u128_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_u128_len,
          str_u128_type,
          str_u128_desc,
          num_u128_min,
          num_u128_max
    );

    // isize (signed int)
    let str_isize_len: String = String::from("Platform");
    let str_isize_type: String = String::from("isize");
    let str_isize_desc: String = String::from("Signed Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_isize_len,
          str_isize_type,
          str_isize_desc,
          str_isize_min,
          str_isize_max
    );

    // usize (unsigned int)
    let str_usize_len: String = String::from("Platform");
    let str_usize_type: String = String::from("usize");
    let str_usize_desc: String = String::from("Unsigned Int");
    println!("{:>10} {:>5} {:>12} {:>45} {:>45}", 
          str_usize_len,
          str_usize_type,
          str_usize_desc,
          str_usize_min,
          str_usize_max
    );

}
