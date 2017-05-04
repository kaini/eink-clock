use rawhw::{ssp, syscon, ioconfig};
use cpu::usleep;

use rawhw::ioconfig::pio0_14 as sck_ioconfig;
use rawhw::ioconfig::pio0_15 as ssel_ioconfig;
use rawhw::ioconfig::pio0_16 as miso_ioconfig;
use rawhw::ioconfig::pio0_17 as mosi_ioconfig;

use rawhw::ioconfig::pio0_27 as ramon_ioconfig;
use rawhw::gpio::gpio0 as ramon_gpio;
const RAMON_BIT: u32 = 27;

const WRITE_CMD: u8 = 0x02;
const READ_CMD: u8 = 0x03;

const X_SIZE: usize = 600;
const Y_SIZE: usize = 800;
const Y_BYTES: usize = Y_SIZE / 8;

static mut CACHED_ADDRESS: u16 = 0xFFFF;
static mut CACHED_VALUE: u8 = 0x00;

pub unsafe fn init() {
    sck_ioconfig::func::set(0x2 /* SSP */);
    sck_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    ssel_ioconfig::func::set(0x2 /* SSP */);
    ssel_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    miso_ioconfig::func::set(0x2 /* SSP */);
    miso_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    mosi_ioconfig::func::set(0x2 /* SSP */);
    mosi_ioconfig::mode::set(ioconfig::Pullup::Disabled);

    ramon_ioconfig::mode::set(ioconfig::Pullup::Disabled);
    ramon_ioconfig::drv::set(true);
    ramon_gpio::dir::set_bit(RAMON_BIT);
    // Reset the RAM.
    ramon_gpio::clr::set(1 << RAMON_BIT);
    usleep(1000);
    ramon_gpio::set::set(1 << RAMON_BIT);

    syscon::sspclkdiv::div::set(2);
    ssp::cr0::scr::set(1);
    ssp::cpsr::cpsdvsr::set(2);
    ssp::cr0::dss::set(7);
    ssp::cr0::cpol::set(true);
    ssp::cr0::cpha::set(true);
    ssp::cr1::sse::set(true);
}

pub fn zero() {
    unsafe {
        write(WRITE_CMD);
        write(0x00);
        write(0x00);
        for _ in 0x0000..0x10000 {
            write(0x00);
        }
        read_until_idle();
        CACHED_VALUE = 0;
    }
}

pub fn set(x: usize, y: usize) {
    let (address, bit) = xy_to_address(x, y);
    unsafe {
        if address != CACHED_ADDRESS {
            flush_cache();
            CACHED_ADDRESS = address;
            refresh_cache();
        }
        CACHED_VALUE |= bit;
    }
}

pub fn get_column(x: usize, dst: &mut [u8]) {
    let (address, _) = xy_to_address(x, 0);
    unsafe {
        if CACHED_ADDRESS != 0xFFFF {
            flush_cache();
            CACHED_ADDRESS = 0xFFFF;
        }

        write(READ_CMD);
        write((address >> 8) as u8);
        write(address as u8);
        write(0x00);
        write(0x00);
        write(0x00);
        write(0x00);
        read_bytes(3);
        for y in 0..(Y_BYTES - 4) {
            write(0x00);
            dst[y] = read_byte();
        }
        dst[Y_BYTES - 4] = read_byte();
        dst[Y_BYTES - 3] = read_byte();
        dst[Y_BYTES - 2] = read_byte();
        dst[Y_BYTES - 1] = read_byte();
    }
}

fn flush_cache() {
    unsafe {
        write(WRITE_CMD);
        write((CACHED_ADDRESS >> 8) as u8);
        write(CACHED_ADDRESS as u8);
        write(CACHED_VALUE);
        read_bytes(4);
    }
}

fn refresh_cache() {
    unsafe {
        write(READ_CMD);
        write((CACHED_ADDRESS >> 8) as u8);
        write(CACHED_ADDRESS as u8);
        write(0x00);
        read_bytes(3);
        CACHED_VALUE = read_byte();
    }
}

unsafe fn read_until_idle() {
    while ssp::sr::bsy::get() {
    }
    while ssp::sr::rne::get() {
        ssp::dr::get();
    }
}

unsafe fn read_byte() -> u8 {
    while !ssp::sr::rne::get() {
    }
    ssp::dr::get() as u8
}

unsafe fn read_bytes(count: usize) {
    for _ in 0..count {
        while !ssp::sr::rne::get() {
        }
        ssp::dr::get();
    }
}

unsafe fn write(data: u8) {
    while !ssp::sr::tnf::get() {
    }
    ssp::dr::set(data as u32);
}

// Returns the byte address and the bit itself (e.g. 0b00010000) to test against.
fn xy_to_address(x: usize, y: usize) -> (u16, u8) {
    return ((x * Y_BYTES + y / 8) as u16, 1 << (y % 8));
}
