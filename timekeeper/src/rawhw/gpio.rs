macro_rules! gpio_register_block {
    ($name:ident, $address:expr) => (
        pub mod $name {
            register_block! {
                mask ($address + 0x00) => { full; }
                pin  ($address + 0x04) => { full; }
                out  ($address + 0x08) => { full; }
                set  ($address + 0x0C) => { full; }
                clr  ($address + 0x10) => { full; }
                not  ($address + 0x14) => { full; }
                dir  ($address + 0x20) => { full; }
                is   ($address + 0x24) => { full; }
                ibe  ($address + 0x28) => { full; }
                iev  ($address + 0x2C) => { full; }
                ie   ($address + 0x30) => { full; }
                ris  ($address + 0x34) => { full; }
                mis  ($address + 0x38) => { full; }
                ic   ($address + 0x3C) => { full; }
            }
        }
    )
}

gpio_register_block!(gpio0, 0x50000000);
gpio_register_block!(gpio1, 0x50010000);
gpio_register_block!(gpio2, 0x50020000);
