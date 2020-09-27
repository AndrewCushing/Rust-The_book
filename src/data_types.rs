pub fn some_types () {
    // We have scalars, which are types that represent a single value:
    let a: i8 = 0; // i for integer
    let b: u8 = 0; // u for unsigned integer
    let c: i16 = 0;
    let d: u16 = 0;
    let e: i32 = 0;
    let f: u32 = 0;
    let g: i64 = 0;
    let h: u64 = 0;

    let bob = 0xfffffffi64; // Type suffice and 0x prefix (meaning this is hex)

    // we can use _ to visually separate a number, without changing its value

    let i:i64 = 100_000_000;

    // arrays

    let a: [i32; 4] = [36, 45, 87, 98];

    // These two are equivalent:
    let b: [i64; 3] = [999; 3];
    let c = [999i64, 999i64, 999i64];


}