register_block! {
    dr 0x40050000 => { full; }
    mr 0x40050004 => { full; }
    lr 0x40050008 => { full; }
    cr 0x4005000C, andmask 1 => {
        0, rtcstart, bool;
    }
    icsc 0x40050010, andmask 1 => {
        0, rtcic, bool;
    }
    ris 0x40050014, andmask 1 => {
        0, rtcris, bool;
    }
    mis 0x40050018, andmask 1 => {
        0, rtcmis, bool;
    }
    icr 0x4005001C, andmask 1 => {
        0, rtcicr, bool;
    }
}