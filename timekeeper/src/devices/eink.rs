use rawhw::ioconfig::Pullup;
use devices::cpu::usleep;
use rawhw;

// ONPOS
use rawhw::gpio::gpio0 as onpos_gpio;
use rawhw::ioconfig::pio0_18 as onpos_ioconfig;
const ONPOS_BIT: u32 = 18;

// ONNEG
use rawhw::gpio::gpio0 as onneg_gpio;
use rawhw::ioconfig::pio0_19 as onneg_ioconfig;
const ONNEG_BIT: u32 = 19;

// DON
use rawhw::gpio::gpio0 as don_gpio;
use rawhw::ioconfig::pio0_28 as don_ioconfig;
const DON_BIT: u32 = 28;

// DD
use rawhw::gpio::gpio0 as dd_gpio;
use rawhw::ioconfig::pio0_0 as dd0_ioconfig;
use rawhw::ioconfig::pio0_1 as dd1_ioconfig;
use rawhw::ioconfig::pio0_2 as dd2_ioconfig;
use rawhw::ioconfig::pio0_3 as dd3_ioconfig;
use rawhw::ioconfig::pio0_4 as dd4_ioconfig;
use rawhw::ioconfig::pio0_5 as dd5_ioconfig;
use rawhw::ioconfig::pio0_6 as dd6_ioconfig;
use rawhw::ioconfig::pio0_7 as dd7_ioconfig;
const DD_SHIFT: u32 = 0;

// DCL
use rawhw::gpio::gpio0 as dcl_gpio;
use rawhw::ioconfig::pio0_8 as dcl_ioconfig;
const DCL_BIT: u32 = 8;

// DLE
use rawhw::gpio::gpio0 as dle_gpio;
use rawhw::ioconfig::pio0_9 as dle_ioconfig;
const DLE_BIT: u32 = 9;

// DOE
use rawhw::gpio::gpio0 as doe_gpio;
use rawhw::ioconfig::pio0_10 as doe_ioconfig;
const DOE_BIT: u32 = 10;

// DGMODE
use rawhw::gpio::gpio0 as dgmode_gpio;
use rawhw::ioconfig::pio0_14 as dgmode_ioconfig;
const DGMODE_BIT: u32 = 14;

// DSPV
use rawhw::gpio::gpio0 as dspv_gpio;
use rawhw::ioconfig::pio0_15 as dspv_ioconfig;
const DSPV_BIT: u32 = 15;

// DCKV
use rawhw::gpio::gpio0 as dckv_gpio;
use rawhw::ioconfig::pio0_16 as dckv_ioconfig;
const DCKV_BIT: u32 = 16;

// DSPH
use rawhw::gpio::gpio0 as dsph_gpio;
use rawhw::ioconfig::pio0_17 as dsph_ioconfig;
const DSPH_BIT: u32 = 17;

const NOOP: u8  = 0b00;
const WHITE: u8 = 0b10;
const BLACK: u8 = 0b01;

pub const SCANLINES: i32 = 600;
pub const SCANLINE_WIDTH: i32 = 800;
pub const BUFFER_BYTES: usize = (SCANLINE_WIDTH / 8) as usize;
static mut BUFFER: [u8; BUFFER_BYTES] = [0; BUFFER_BYTES];

macro_rules! setup_output {
    ($ioconfig:ident, $gpio:ident, $bit:expr) => ({
        $ioconfig::mode::set(Pullup::Disabled);
        $gpio::dir::set_bit($bit);
        $gpio::clr::set(1 << $bit);
    });
}

pub struct Eink {
    _phantom: (),
}

impl Eink {
    pub unsafe fn new() -> Eink {
        setup_output!(onpos_ioconfig, onpos_gpio, ONPOS_BIT);
        setup_output!(onneg_ioconfig, onneg_gpio, ONNEG_BIT);
        setup_output!(don_ioconfig, don_gpio, DON_BIT);
        don_ioconfig::drv::set(true);
        setup_output!(dd0_ioconfig, dd_gpio, DD_SHIFT + 0);
        setup_output!(dd1_ioconfig, dd_gpio, DD_SHIFT + 1);
        setup_output!(dd2_ioconfig, dd_gpio, DD_SHIFT + 2);
        setup_output!(dd3_ioconfig, dd_gpio, DD_SHIFT + 3);
        setup_output!(dd4_ioconfig, dd_gpio, DD_SHIFT + 4);
        setup_output!(dd5_ioconfig, dd_gpio, DD_SHIFT + 5);
        setup_output!(dd6_ioconfig, dd_gpio, DD_SHIFT + 6);
        setup_output!(dd7_ioconfig, dd_gpio, DD_SHIFT + 7);
        setup_output!(dcl_ioconfig, dcl_gpio, DCL_BIT);
        setup_output!(dle_ioconfig, dle_gpio, DLE_BIT);
        doe_gpio::dir::set_bit(DOE_BIT);
        doe_gpio::clr::set(1 << DOE_BIT);
        setup_output!(dgmode_ioconfig, dgmode_gpio, DGMODE_BIT);
        setup_output!(dspv_ioconfig, dspv_gpio, DSPV_BIT);
        setup_output!(dckv_ioconfig, dckv_gpio, DCKV_BIT);
        setup_output!(dsph_ioconfig, dsph_gpio, DSPH_BIT);

        Eink {
            _phantom: (),
        }
    }

    pub fn enable(&mut self) {
        unsafe {
            don_gpio::set::set(1 << DON_BIT);
            usleep(100);
            onneg_gpio::set::set(1 << ONNEG_BIT);
            usleep(1000);
            onpos_gpio::set::set(1 << ONPOS_BIT);
            usleep(1000);
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            onpos_gpio::clr::set(1 << ONPOS_BIT);
            usleep(1000);
            onneg_gpio::clr::set(1 << ONNEG_BIT);
            usleep(1000);
            don_gpio::clr::set(1 << DON_BIT);
            usleep(100);
        }
    }

    unsafe fn draw_mode_on(&mut self) {
        dgmode_gpio::set::set(1 << DGMODE_BIT);
        dckv_gpio::set::set(1 << DCKV_BIT);
        dsph_gpio::set::set(1 << DSPH_BIT);
        usleep(1000);
    }

    unsafe fn draw_mode_off(&mut self) {
        dd_gpio::clr::set(0xFF << DD_SHIFT);
        dcl_gpio::clr::set(1 << DCL_BIT);
        dle_gpio::clr::set(1 << DLE_BIT);
        doe_gpio::clr::set(1 << DOE_BIT);
        dgmode_gpio::clr::set(1 << DGMODE_BIT);
        dspv_gpio::clr::set(1 << DSPV_BIT);
        dckv_gpio::clr::set(1 << DCKV_BIT);
        dsph_gpio::clr::set(1 << DSPH_BIT);
    }

    unsafe fn advance_line(&mut self) {
        dckv_gpio::clr::set(1 << DCKV_BIT);
        usleep(1);
        dckv_gpio::set::set(1 << DCKV_BIT);
        usleep(1);
    }

    unsafe fn begin_frame(&mut self) {
        dspv_gpio::set::set(1 << DSPV_BIT);
        usleep(500);
        dspv_gpio::clr::set(1 << DSPV_BIT);
        usleep(1);
        dckv_gpio::clr::set(1 << DCKV_BIT);
        usleep(25);
        dckv_gpio::set::set(1 << DCKV_BIT);
        usleep(1);
        dspv_gpio::set::set(1 << DSPV_BIT);
        usleep(25);

        // For some reason I have to advance 3 times to fill the whole screen
        self.advance_line();
        self.advance_line();
        self.advance_line();
    }

    unsafe fn end_frame(&mut self) {
    }

    unsafe fn begin_line(&mut self) {
        doe_gpio::set::set(1 << DOE_BIT);
        dsph_gpio::clr::set(1 << DSPH_BIT);
        usleep(1);
    }

    unsafe fn end_line(&mut self) {
        dsph_gpio::set::set(1 << DSPH_BIT);
        usleep(1);

        dcl_gpio::set::set(1 << DCL_BIT);
        dckv_gpio::clr::set(1 << DCKV_BIT);
        usleep(1);
        dcl_gpio::clr::set(1 << DCL_BIT);
        usleep(1);
        dckv_gpio::set::set(1 << DCKV_BIT);
        usleep(1);
        doe_gpio::clr::set(1 << DOE_BIT);
        usleep(200);
        doe_gpio::set::set(1 << DOE_BIT);
        usleep(1);
        dle_gpio::set::set(1 << DLE_BIT);
        usleep(1);
        dle_gpio::clr::set(1 << DLE_BIT);
        usleep(1);
        
        doe_gpio::clr::set(1 << DOE_BIT);
    }

    // Use the constants NOOP, WHITE, BLACK.
    unsafe fn set_four_pixels(&mut self, pixels: &[u8; 4]) {
        dd_gpio::mask::set(!(0xFF << DD_SHIFT));
        dd_gpio::out::set((((pixels[0] << 6) | (pixels[1] << 4) | (pixels[2] << 2) | pixels[3]) as u32) << DD_SHIFT);
        dd_gpio::mask::set(0);
        dcl_gpio::set::set(1 << DCL_BIT);
        usleep(1);
        dcl_gpio::clr::set(1 << DCL_BIT);
        usleep(1);
    }

    unsafe fn clear(&mut self) {
        self.begin_frame();
        for _ in 0..600 {
            self.begin_line();
            for _ in 0..200 {
                self.set_four_pixels(&[WHITE; 4]);
            }
            self.end_line();
        }
        self.end_frame();
    }

    pub fn render<Draw>(&mut self, clear: bool, mut draw: Draw)
            where Draw: FnMut(i32, &mut [u8; BUFFER_BYTES]) {
        unsafe {
            self.draw_mode_on();

            if clear {
                self.clear();
            }

            self.begin_frame();
            for scanline in 0..SCANLINES {
                draw(scanline, &mut BUFFER);
                let mut buffer_byte = 0;

                self.begin_line();
                for _ in 0..(SCANLINE_WIDTH / 8) {
                    self.set_four_pixels(&[
                        if (BUFFER[buffer_byte] & (1 << 0)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 1)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 2)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 3)) != 0 { BLACK } else { WHITE },
                    ]);
                    self.set_four_pixels(&[
                        if (BUFFER[buffer_byte] & (1 << 4)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 5)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 6)) != 0 { BLACK } else { WHITE },
                        if (BUFFER[buffer_byte] & (1 << 7)) != 0 { BLACK } else { WHITE },
                    ]);
                    buffer_byte += 1;
                }
                self.end_line();
            }
            self.end_frame();

            self.draw_mode_off();
        }
    }
}

