pub const CT16B0: i32 = 13;
pub const RTC: i32 = 30;

register_block! {
    iser 0xE000E100 => { full; }
    icer 0xE000E180 => { full; }
    ispr 0xE000E200 => { full; }
    icpr 0xE000E280 => { full; }

    scr 0xE000ED10 => {
        4, sevonpend, bool;
        2, sleepdeep, bool;
        1, sleeponexit, bool;
    }
}
