/// Grammar:
/// 
/// START = "register_block!" "{" REGISTER* "}"
/// REGISTER = NAME ADDRESS ["," "andmask" ANDMASK] "=>" "{" FIELD* ";" "}"
/// FIELD = "full" | START ["," END] "," NAME "," TYPE
///
/// Where NAME and TYPE is a Rust identifier. ADDRESS, ANDMASK, START and END
/// are Rust expressions.
macro_rules! register_block {
    () => ();
    (
        $name:ident $address:expr => { $($($inner:tt),+;)* } $($rest:tt)*
    ) => (
        pub mod $name {
            #[allow(unused_imports)]
            use super::*;
            $(register_accessor!($address, !0, $($inner),*);)*
        }
        register_block!($($rest)*);
    );
    (
        $name:ident $address:expr, andmask $andmask:expr => { $($($inner:tt),+;)* } $($rest:tt)*
    ) => (
        pub mod $name {
            #[allow(unused_imports)]
            use super::*;
            $(register_accessor!($address, $andmask, $($inner),*);)*
        }
        register_block!($($rest)*);
    );
}

macro_rules! register_accessor {
    // Accessors for a value that spans multiple bits
    ($address:expr, $andmask:expr, $bit_from:expr, $bit_to:expr, $field:ident, $field_type:ident) => (
        pub mod $field {
            #[allow(unused_imports)]
            use super::*;

            #[allow(dead_code)]
            pub unsafe fn set(value: $field_type) {
                let mut reg_value = ::core::ptr::read_volatile($address as *const u32) & $andmask;
                reg_value &= !(((1 << ($bit_to - $bit_from + 1)) - 1) << $bit_from);
                reg_value |= ((value as u32) & ((1 << ($bit_to - $bit_from + 1)) - 1)) << $bit_from;
                ::core::ptr::write_volatile($address as *mut u32, reg_value & $andmask);
            }

            #[allow(dead_code)]
            pub unsafe fn get() -> $field_type {
                let mut value = ::core::ptr::read_volatile($address as *const u32) & $andmask;
                value >>= $bit_from;
                value &= (1 << ($bit_to - $bit_from + 1)) - 1;
                register_bits_from_u32!($field_type, value)
            }

            #[allow(dead_code)]
            pub unsafe fn set_fullreg_zero_this_one() {
                ::core::ptr::write_volatile($address as *mut u32, (((1 << ($bit_to - $bit_from + 1)) - 1) << $bit_from) & $andmask)
            }
        }
    );
    // Accessors for a single bit value.
    ($address:expr, $andmask:expr, $bit:expr, $field:ident, $field_type:ident) => (
        register_accessor!($address, $andmask, $bit, $bit, $field, $field_type);
    );
    // Accessors for a full register value (32 bit).
    ($address:expr, $andmask:expr, full) => (
        #[allow(dead_code)]
        pub unsafe fn get() -> u32 {
            ::core::ptr::read_volatile($address as *const u32) & $andmask
        }

        #[allow(dead_code)]
        pub unsafe fn set(value: u32) {
            ::core::ptr::write_volatile($address as *mut u32, value & $andmask)
        }

        #[allow(dead_code)]
        pub unsafe fn set_bit(index: u32) {
            set(get() | (1 << index));
        }

        #[allow(dead_code)]
        pub unsafe fn clear_bit(index: u32) {
            set(get() & !(1 << index));
        }
    );
}

macro_rules! register_bits_from_u32 {
    (u8, $what:expr) => ($what as u8);
    (u16, $what:expr) => ($what as u16);
    (bool, $what:expr) => ($what != 0);
    ($to:ident, $what:expr) => (::core::mem::transmute::<u32, $to>($what));
}

#[cfg(test)]
mod test {
    static mut SET_V: u32 = 0b1;
    static mut GET_V: u32 = 0b1101000000000000;

    register_block! {
        set_v (&mut SET_V) => {
            2,4, test, u32;
            30, late, u32;
        }
        get_v (&mut GET_V) => {
            12,20, test, u32;
        }
    }

    #[test]
    fn test_set() {
        unsafe {
            set_v::test::set(2);
            assert_eq!(SET_V, 0b1001);

            set_v::late::set(7);
            assert_eq!(SET_V, 0b1000000000000000000000000001001);

            set_v::late::set_fullreg_zero_this_one();
            assert_eq!(SET_V, 0b1000000000000000000000000000000);
        }
    }

    #[test]
    fn test_get() {
        unsafe {
            assert_eq!(get_v::test::get(), 0b1101);
        }
    }
}
