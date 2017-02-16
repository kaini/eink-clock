macro_rules! counter_register {
    ($name:ident, $address:expr) => (
        pub mod $name {
            register_block! {
                ir   ($address + 0x00) => {
                    0, mr0int, bool;
                    1, mr1int, bool;
                    2, mr2int, bool;
                    3, mr3int, bool;
                    4, cr0int, bool;
                    5, cr1int, bool;
                    6, cr2int, bool;
                    7, cr3int, bool;
                }
                tcr  ($address + 0x04) => {
                    0, cen, bool;
                    1, crst, bool;
                }
                tc   ($address + 0x08) => {
                    0,15, tc, u16;
                }
                pr   ($address + 0x0C) => {
                    0,15, pcval, u16;
                }
                pc   ($address + 0x10) => {
                    0,15, pc, u16;
                }
                mcr  ($address + 0x14) => {
                    0, mr0i, bool;
                    1, mr0r, bool;
                    2, mr0s, bool;
                    3, mr1i, bool;
                    4, mr1r, bool;
                    5, mr1s, bool;
                    6, mr2i, bool;
                    7, mr2r, bool;
                    8, mr2s, bool;
                    9, mr3i, bool;
                    10, mr3r, bool;
                    11, mr3s, bool;
                }
                mr0  ($address + 0x18) => {
                    0,15, match_, u16;
                }
                mr1  ($address + 0x1C) => {
                    0,15, match_, u16;
                }
                mr2  ($address + 0x20) => {
                    0,15, match_, u16;
                }
                mr3  ($address + 0x24) => {
                    0,15, match_, u16;
                }
                ccr  ($address + 0x28) => {
                    0, cap0re, bool;
                    1, cap0fe, bool;
                    2, cap0i, bool;
                    3, cap1re, bool;
                    4, cap1fe, bool;
                    5, cap1i, bool;
                    6, cap2re, bool;
                    7, cap2fe, bool;
                    8, cap2i, bool;
                    9, cap3re, bool;
                    10, cap3fe, bool;
                    11, cap3i, bool;
                }
                cr0  ($address + 0x2C) => {
                    0,15, cap, u16;
                }
                cr1  ($address + 0x30) => {
                    0,15, cap, u16;
                }
                cr2  ($address + 0x34) => {
                    0,15, cap, u16;
                }
                cr3  ($address + 0x38) => {
                    0,15, cap, u16;
                }
                // TODO emr  ($address + 0x3C) => { }
                // TODO ctcr ($address + 0x70) => { }
                // TODO pwmc ($address + 0x74) => { }
            }
        }
    )
}

counter_register!(ct16b0, 0x40010000);
counter_register!(ct16b1, 0x40014000);
