use struct_packer::{pack_struct};

#[test]
fn pack_unpack_single_u8()
{
    enum SomeEnum {
        VarA(),
        VarB(u16)
    }

    /*#[pack_struct]
    struct SomeStruct {
        a: u8,
        e1: SomeEnum,
        e2: SomeEnum
    }*/

    static DATA_SIZE : usize = std :: mem :: size_of :: < u8 > () + std :: mem :: size_of :: < SomeEnum > () + std :: mem :: size_of :: < SomeEnum > () ;
    static DATA_SIZE_ROUNDED : usize = f64::powf(2.0, ((DATA_SIZE as f64).ln() / (2.0f64).ln()).ceil()) as usize;
    //const DATA_SIZE_ROUNDED : usize = usize :: pow(2f64, ((DATA_SIZE as f64).log() / 2f64.log()).ceil());

    struct SomeStruct { a : u8, e1 : SomeEnum, e2 : SomeEnum }
    struct SomeStructPacked
    {

    }

    //let s: SomeStruct = SomeStruct{a: 15, e1: SomeEnum::VarA(), e2: SomeEnum::VarB(1234)};
    //let s_packed: SomeStructPacked = s.pack();
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