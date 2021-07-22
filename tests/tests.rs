use struct_packer::{pack_struct};

#[test]
fn pack_unpack_single_u8()
{
    #[pack_struct]
    #[derive(Default, Debug)]
    struct SomeStruct {
        a: u8,
        b: u16,
        d: u32
    }

    let s: SomeStruct = SomeStruct{a: 15, b: 123, d: 23363};
    let s_packed: SomeStructPacked = s.pack();
    let s_unpacked: SomeStruct = s_packed.unpack();

    eprintln!("Before pack: {:?}", s);
    eprintln!("Packed: {}", s_packed.data);
    eprintln!("Unpacked: {:?}", s_unpacked);

}


// Test primitive types
    // Test packing and unpacking and that values remain the same: one unsigned integer field
    // Test packing and unpacking and that values remain the same: multiple unsigned integer fields
    // Test packing and unpacking and that values remain the same: multiple non-unsigned integer fields

// Test enums
    // Test packing and unpacking and that values remain the same: one enum field
    // Test packing and unpacking and that values remain the same: multiple enum fields

// Test failure conditions
    // Test that compilation fails if all fields cant be packed into one field due to too big size
    // Test that compilation fails using collection types
    // Test that compilation fails using non-value fields such as references or pointers

// Test recursion
    // Test recursive structs
    // Test recursive enums
    // Test recursive and non-recursive structs
    // test recursive and non-recursive enums

// Test packing order
    // Test that fields first-last are packed in order most_significant_bits - least_significant_bits
    // Test sorting packed structs of primitive types
    // Test sorting packed structs of enums

// Other
    // Test unnamed fields for tuple structs