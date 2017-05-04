register_block! {
    csr 0xE000E010, andmask 0b10000000000000111 => {
        0, enable, bool;
        1, tickint, bool;
        16, countflag, bool;
    }
    rvr 0xE000E014, andmask 0xFFFFFF  => {
        0,23, reload, u32;
    }
    cvr 0xE000E018, andmask 0xFFFFFF => {
        0,23, current, u32;
    }
}
