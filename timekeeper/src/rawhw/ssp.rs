#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum FrameFormat {
    Spi = 0,
    Ti = 1,
    Microwire = 2,
}

register_block! {
    cr0 0x40040000 => {
        0,3, dss, u8;
        4,5, frf, FrameFormat;
        6, cpol, bool;
        7, cpha, bool;
        8,15, scr, u8;
    }
    cr1 0x40040004, andmask 0b1111 => {
        0, lbm, bool;
        1, sse, bool;
        2, ms, bool;
        3, sod, bool;
    }
    dr 0x40040008 => {
        // Strictly speaking this field is only from bit 0 to 15, but the
        // read has a side effect and set on a partial field would perform
        // such a read.
        full;
    }
    sr 0x4004000C, andmask 0b11111 => {
        0, tfe, bool;
        1, tnf, bool;
        2, rne, bool;
        3, rff, bool;
        4, bsy, bool;
    }
    cpsr 0x40040010 => {
        0,7, cpsdvsr, u8;
    }
    imsc 0x40040014, andmask 0b1111 => {
        0, rorim, bool;
        1, rtim, bool;
        2, rxim, bool;
        3, txim, bool;
    }
    ris 0x40040018, andmask 0b1111 => {
        0, rorris, bool;
        1, rtris, bool;
        2, rxris, bool;
        3, txris, bool;
    }
    mis 0x4004001C, andmask 0b1111 => {
        0, rormis, bool;
        1, rtmis, bool;
        2, rxmis, bool;
        3, txmis, bool;
    }
    icr 0x40040020, andmask 0b11 => {
        0, roric, bool;
        1, rtic, bool;
    }
    dmacr 0x40040024, andmask 0b11 => {
        0, rxdmae, bool;
        1, txdmae, bool;
    }
}
