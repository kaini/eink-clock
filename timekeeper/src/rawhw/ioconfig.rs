#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum Mode {
    Analog = 0,
    Digital = 1,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum Pullup {
    Disabled = 0,
    Enabled = 1,
}

macro_rules! ioconfig_register {
    ($name:ident, $address:expr) => (
        register_block! {
            $name (0x40044000 + $address) => {
                0,2, func, u8;
                4, mode, Pullup;
                6, inv, bool;
                9, drv, bool;
                10, od, bool;
                11,12, s_mode, u8;
                13,15, clk_div, u8;
            }
        }
    );
    ($name:ident, $address:expr, ad) => (
        register_block! {
            $name (0x40044000 + $address) => {
                0,2, func, u8;
                4, mode, Pullup;
                6, inv, bool;
                7, admode, Mode;
                9, drv, bool;
                10, od, bool;
                11,12, s_mode, u8;
                13,15, clk_div, u8;
            }
        }
    );
    ($name:ident, $address:expr, i2c) => (
        register_block! {
            $name (0x40044000 + $address) => {
                0,2, func, u8;
                6, inv, bool;
                10, tod, bool;
                11,12, s_mode, u8;
                13,15, clk_div, u8;
            }
        }
    );
}

ioconfig_register!(pio0_0, 0x044);
ioconfig_register!(pio0_1, 0x048);
ioconfig_register!(pio0_2, 0x04C);
ioconfig_register!(pio0_3, 0x054);
ioconfig_register!(pio0_4, 0x058);
ioconfig_register!(pio0_5, 0x05C);
ioconfig_register!(pio0_6, 0x060);
ioconfig_register!(pio0_7, 0x064);
ioconfig_register!(pio0_8, 0x068);
ioconfig_register!(pio0_9, 0x06C);
ioconfig_register!(pio0_10, 0x090, i2c);
ioconfig_register!(pio0_11, 0x094, i2c);
ioconfig_register!(pio0_12, 0x098);
ioconfig_register!(pio0_13, 0x09C);
ioconfig_register!(pio0_14, 0x0A0);
ioconfig_register!(pio0_15, 0x0A4);
ioconfig_register!(pio0_16, 0x0A8);
ioconfig_register!(pio0_17, 0x0AC);
ioconfig_register!(pio0_18, 0x0B0);
ioconfig_register!(pio0_19, 0x008, ad);
ioconfig_register!(pio0_20, 0x00C, ad);
ioconfig_register!(pio0_21, 0x010, ad);
ioconfig_register!(pio0_22, 0x014, ad);
ioconfig_register!(pio0_23, 0x018, ad);
ioconfig_register!(pio0_24, 0x01C, ad);
ioconfig_register!(pio0_25, 0x020, ad);
ioconfig_register!(pio0_26, 0x024, ad);
ioconfig_register!(pio0_27, 0x028);
ioconfig_register!(pio0_28, 0x03C);
ioconfig_register!(pio0_29, 0x040);
ioconfig_register!(pio0_30, 0x0B4, ad);
ioconfig_register!(pio0_31, 0x0B8, ad);
