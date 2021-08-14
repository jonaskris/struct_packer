use struct_packer::{pack_struct};

#[test]
fn pack_unpack_single_unsigned()
{
    #[pack_struct]
    #[derive(Default)]
    struct TestStructA {
        a: u8
    }

    let s: TestStructA = TestStructA{a: 15};
    let s_packed: TestStructAPacked = s.pack();
    let s_unpacked: TestStructA = s_packed.unpack();

    assert_eq!(s.a, s_unpacked.a);
    assert_eq!(1, std::mem::size_of::<TestStructAPacked>());
}

#[test]
fn pack_unpack_multiple_unsigned()
{
    #[pack_struct]
    #[derive(Default)]
    struct TestStructB {
        a: u8,
        b: u32,
        c: u8,
        d: u16
    }

    let s: TestStructB = TestStructB{a: 15, b: 123534, c: 213, d: 64253};
    let s_packed: TestStructBPacked = s.pack();
    let s_unpacked: TestStructB = s_packed.unpack();

    assert_eq!(s.a, s_unpacked.a);
    assert_eq!(s.b, s_unpacked.b);
    assert_eq!(s.c, s_unpacked.c);
    assert_eq!(s.d, s_unpacked.d);
    assert_eq!(8, std::mem::size_of::<TestStructBPacked>());
}

#[test]
fn pack_unpack_multiple_mixed()
{
    #[pack_struct]
    #[derive(Default)]
    struct TestStructC {
        a: u8,
        b: char,
        c: i32,
        d: bool,
        e: f32
    }

    let s: TestStructC = TestStructC{a: 15, b: 'b', c: -312, d: true, e: 15.4};
    let s_packed: TestStructCPacked = s.pack();
    let s_unpacked: TestStructC = s_packed.unpack();

    assert_eq!(s.a, s_unpacked.a);
    assert_eq!(s.b, s_unpacked.b);
    assert_eq!(s.c, s_unpacked.c);
    assert_eq!(s.d, s_unpacked.d);
    assert_eq!(s.e, s_unpacked.e);
    assert_eq!(16, std::mem::size_of::<TestStructCPacked>());
}

#[test]
fn pack_unpack_multiple_bool()
{
    #[pack_struct]
    #[derive(Default)]
    struct TestStructD {
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        e: bool
    }

    let s: TestStructD = TestStructD{a: true, b: false, c: true, d: true, e: false};
    let s_packed: TestStructDPacked = s.pack();
    let s_unpacked: TestStructD = s_packed.unpack();

    assert_eq!(s.a, s_unpacked.a);
    assert_eq!(s.b, s_unpacked.b);
    assert_eq!(s.c, s_unpacked.c);
    assert_eq!(s.d, s_unpacked.d);
    assert_eq!(s.e, s_unpacked.e);
    assert_eq!(1, std::mem::size_of::<TestStructDPacked>());
}


// Test failure conditions
    // Test that compilation fails if all fields cant be packed into one field due to too big size
    // Test that compilation fails using collection types
    // Test that compilation fails using non-value fields such as references or pointers

// Test packing order
    // Test that fields first-last are packed in order most_significant_bits - least_significant_bits
    // Test sorting packed structs of primitive types
    // Test sorting packed structs of enums

// Other
    // Test unnamed fields for tuple structs